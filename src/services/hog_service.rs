use crate::errors::SomeCreateError;
use crate::models::hog::Hog;
use crate::models::hog_client_schema::HogRequest;
use crate::models::hog_record::HogRecord;
use crate::models::options::{self, build_log_data_value_aggregation_pipeline, OptionsRequest};
use crate::models::statistics::HogStatistics;
use crate::utils::utils;
use futures::{StreamExt, TryStreamExt};
use lapin::{BasicProperties, Channel};
use mongodb::options::FindOptions;
use mongodb::{bson::doc, Collection, Database};

use serde_json::Map;
use uuid::Uuid;

pub struct HogService {
    db: Database,
    collection: Collection<HogRecord>,
    rabbit_channel: Channel,
}

impl HogService {
    pub fn new(db: &Database, rabbit_channel: Channel) -> Self {
        let collection = db.collection::<HogRecord>("hog");
        HogService {
            db: db.clone(),
            collection,
            rabbit_channel,
        }
    }

    pub async fn create_hog(&self, req: HogRequest) -> Result<Hog, SomeCreateError> {
        let uuid = Uuid::new_v4();

        let hog_record = HogRecord {
            log_timestamp: utils::rfc3339_str_to_bson(&req.log_timestamp).unwrap(),
            log_level: req.log_level,
            log_message: req.log_message,
            log_data: req.log_data,
            log_type: req.log_type,
            log_source: req.log_source,
            log_source_id: req.log_source_id,
            hog_uuid: Some(uuid.to_string()),
            hog_timestamp: Some(utils::convert_timestamp_chrono_to_bson(
                utils::get_timestamp(),
            )),
            created_at: None,
            id: None,
        };

        let payload =
            serde_json::to_vec(&hog_record).map_err(|e| SomeCreateError::new(e.to_string()))?;

        self.rabbit_channel
            .basic_publish(
                "",
                "hog_queue",
                lapin::options::BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| SomeCreateError::new(e.to_string()))?
            .await
            .map_err(|e| SomeCreateError::new(e.to_string()))?;

        let hog = utils::convert_hog_record_to_hog(&hog_record);

        Ok(hog)
    }

    pub async fn get_hogs(&self) -> Result<Vec<Hog>, mongodb::error::Error> {
        let filter = doc! {};
        let find_options = FindOptions::builder()
            .limit(1000)
            .sort(doc! {"hog_timestamp": -1})
            .build();

        let mut cursor = self
            .collection
            .find(filter)
            .with_options(find_options)
            .await?;

        let mut hog_records: Vec<HogRecord> = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(hog_record) => hog_records.push(hog_record),
                Err(e) => return Err(e.into()),
            }
        }

        let hogs = utils::convert_hog_records_to_hogs(hog_records);

        Ok(hogs)
    }

    pub async fn search_hogs(
        &self,
        options: OptionsRequest,
    ) -> Result<Vec<Hog>, mongodb::error::Error> {
        let need_aggregation = options.log_data_value.is_some() && options.log_data_field.is_none();

        if need_aggregation && options.log_data_value.is_some() {
            // Use aggregation pipeline
            let log_data_value = options
                .log_data_value
                .as_ref()
                .expect("log_data_value must be Some");
            let pipeline = build_log_data_value_aggregation_pipeline(
                log_data_value
                    .as_ref()
                    .expect("log_data_value must be Some"),
                &options,
            );

            let mut cursor = self.collection.aggregate(pipeline).await?;

            let mut hog_records: Vec<HogRecord> = Vec::new();

            while let Some(doc) = cursor.try_next().await? {
                let hog: HogRecord = bson::from_document(doc)?;
                hog_records.push(hog);
            }

            let hogs = utils::convert_hog_records_to_hogs(hog_records);

            return Ok(hogs);
        } else {
            let filter = options::build_filter(&options.clone());
            let limit = options.hog_limit.unwrap_or(1000);

            let find_options = FindOptions::builder()
                .limit(limit)
                .sort(doc! { "_id": -1 })
                .build();

            let mut cursor = self
                .collection
                .find(filter)
                .with_options(find_options)
                .await?;

            let mut hog_records = Vec::new();

            while let Some(result) = cursor.next().await {
                match result {
                    Ok(hog_record) => hog_records.push(hog_record),
                    Err(e) => return Err(e.into()),
                }
            }

            let hogs = utils::convert_hog_records_to_hogs(hog_records);

            Ok(hogs)
        }
    }

    pub async fn hog_stats(&self) -> Result<Map<String, serde_json::Value>, mongodb::error::Error> {
        let stats = self.db.run_command(doc! { "collStats": "hog" }).await?;
        // parse the stats into a more usable format
        let stats_doc = bson::from_document::<bson::Document>(stats)?;
        // further process the stats to retrieve record count, index sizes, etc.
        let record_count = stats_doc
            .get_i64("count")
            .unwrap_or(0);
        let default_doc = doc! {};
        let index_sizes = stats_doc
            .get_document("indexSizes")
            .unwrap_or(&default_doc);
        let index_sizes_parsed: Map<String, serde_json::Value> = index_sizes
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    serde_json::to_value(v.as_i64().unwrap_or(0)).unwrap_or(serde_json::Value::Null),
                )
            })
            .collect();

        let mut stats_parsed = Map::new();
        stats_parsed.insert("record_count".to_string(), serde_json::Value::from(record_count));
        stats_parsed.insert("index_sizes".to_string(), serde_json::Value::Object(index_sizes_parsed));
        Ok(stats_parsed)
    }

    pub async fn hog_statistics(&self) -> Result<HogStatistics, mongodb::error::Error> {
        let count = |filter| self.collection.count_documents(filter);
        Ok(HogStatistics {
            record_count: count(doc! {}).await.unwrap_or(0) as i64,
            log_level_count: count(doc! { "log_level": { "$exists": true, "$ne": null } })
                .await
                .unwrap_or(0) as i64,
            log_type_count: count(doc! { "log_type": { "$exists": true, "$ne": null } })
                .await
                .unwrap_or(0) as i64,
            log_source_count: count(doc! { "log_source": { "$exists": true, "$ne": null } })
                .await
                .unwrap_or(0) as i64,
            log_source_id_count: count(doc! { "log_source_id": { "$exists": true, "$ne": null } })
                .await
                .unwrap_or(0) as i64,
            log_data_count: count(doc! { "log_data": { "$exists": true, "$ne": null } })
                .await
                .unwrap_or(0) as i64,
            log_timestamp_count: count(doc! { "log_timestamp": { "$exists": true, "$ne": null } })
                .await
                .unwrap_or(0) as i64,
            log_message_count: count(doc! { "log_message": { "$exists": true, "$ne": null } })
                .await
                .unwrap_or(0) as i64,
        })
    }
}
