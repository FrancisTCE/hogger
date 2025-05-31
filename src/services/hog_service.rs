use crate::errors::SomeCreateError;
use crate::models::client_request::ClientRequest;
use crate::models::hog::Hog;
use crate::models::hog_client_schema::HogRequest;
use crate::models::options::{self, OptionsRequest, build_log_data_value_aggregation_pipeline};
use crate::models::statistics::HogStatistics;
use crate::utils::utils;
use futures::{StreamExt, TryStreamExt};
use lapin::{BasicProperties, Channel};
use mongodb::options::FindOptions;
use mongodb::{Collection, Database, bson::doc};

use uuid::Uuid;

pub struct HogService {
    collection: Collection<Hog>,
    rabbit_channel: Channel,
}

impl HogService {
    pub fn new(db: &Database, rabbit_channel: Channel) -> Self {
        let collection = db.collection::<Hog>("hog");
        HogService {
            collection,
            rabbit_channel,
        }
    }

    pub async fn create_hog(&self, client_request: ClientRequest) -> anyhow::Result<Hog> {
        let uuid = Uuid::new_v4();

        let timestamp = utils::get_timestamp();

        let hog = Hog::new(
            client_request,
            Some(uuid.to_string()),
            Some(timestamp),
            None,
        );

        let payload = serde_json::to_vec(&hog)?;

        self.rabbit_channel
            .basic_publish(
                "",
                "hog_queue",
                lapin::options::BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;

        Ok(hog)
    }

    pub async fn create_hog_validated(&self, req: HogRequest) -> Result<Hog, SomeCreateError> {
    let uuid = Uuid::new_v4();
    let timestamp = utils::get_timestamp();

    let client_request = ClientRequest {
        log_timestamp: req.log_timestamp,
        log_level: req.log_level,
        log_message: req.log_message,
        log_data: req.log_data,
        log_type: req.log_type,
        log_source: req.log_source,
        log_source_id: req.log_source_id,
    };

    let hog = Hog::new(
        client_request,
        Some(uuid.to_string()),
        Some(timestamp),
        None,
    );

    let payload = serde_json::to_vec(&hog).map_err(|e| SomeCreateError::new(e.to_string()))?;

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

    Ok(hog)
}
    

    pub async fn get_hogs(&self) -> Result<Vec<Hog>, mongodb::error::Error> {
        let filter = doc! {};
        let find_options = FindOptions::builder().limit(10000).build();
        let mut cursor = self
            .collection
            .find(filter)
            .with_options(find_options)
            .await?;

        let mut hogs = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(hog) => hogs.push(hog),
                Err(e) => return Err(e.into()),
            }
        }

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

            let mut hogs = Vec::new();

            while let Some(doc) = cursor.try_next().await? {
                let hog: Hog = bson::from_document(doc)?;
                hogs.push(hog);
            }

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

            let mut hogs = Vec::new();

            while let Some(result) = cursor.next().await {
                match result {
                    Ok(hog) => hogs.push(hog),
                    Err(e) => return Err(e.into()),
                }
            }
            Ok(hogs)
        }
    }

    pub async fn hog_statistics(&self) -> Result<HogStatistics, mongodb::error::Error> {
        let count = |filter| self.collection.count_documents(filter);
        Ok(HogStatistics {
            record_count: count(doc! {}).await.unwrap_or(0) as i64,
            log_level_count: count(doc! { "log_level": { "$exists": true, "$ne": null } }).await.unwrap_or(0) as i64,
            log_type_count: count(doc! { "log_type": { "$exists": true, "$ne": null } }).await.unwrap_or(0) as i64,
            log_source_count: count(doc! { "log_source": { "$exists": true, "$ne": null } }).await.unwrap_or(0) as i64,
            log_source_id_count: count(doc! { "log_source_id": { "$exists": true, "$ne": null } }).await.unwrap_or(0) as i64,
            log_data_count: count(doc! { "log_data": { "$exists": true, "$ne": null } }).await.unwrap_or(0) as i64,
            log_timestamp_count: count(doc! { "log_timestamp": { "$exists": true, "$ne": null } }).await.unwrap_or(0) as i64,
            log_message_count: count(doc! { "log_message": { "$exists": true, "$ne": null } }).await.unwrap_or(0) as i64,
        })

    }
}
