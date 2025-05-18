use crate::models::client_request::{ClientRequest, ClientRequestWithDateTime};
use crate::models::hog::Hog;
use crate::models::options::{self, OptionsRequest};
use crate::utils::utils;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use lapin::{BasicProperties, Channel};
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

    // Parse string to DateTime<Utc>
    let parsed_log_timestamp: DateTime<Utc> = client_request
        .log_timestamp
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse log_timestamp: {}", e))?;

    // Now create a new ClientRequest with the parsed DateTime instead of string
    let client_request_with_dt = ClientRequestWithDateTime {
        log_timestamp: parsed_log_timestamp,
        log_level: client_request.log_level,
        log_message: client_request.log_message,
        log_data: client_request.log_data,
        log_type: client_request.log_type,
        log_source: client_request.log_source,
        log_source_id: client_request.log_source_id,
    };

    // Generate current timestamp for hog_timestamp
    let timestamp = utils::get_timestamp();

    let hog = Hog::new(
        client_request_with_dt,
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
    pub async fn get_hogs(&self) -> Result<Vec<Hog>, mongodb::error::Error> {
        let filter = doc! {};
        let mut cursor = self.collection.find(filter).await?;
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
        let filter = options::build_filter(&options.clone());
        let mut cursor = self.collection.find(filter).with_options(None).await?;
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
