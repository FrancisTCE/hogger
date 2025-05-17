
use crate::models::client_request::ClientRequest;
use crate::models::options::{self, OptionsRequest};
use crate::models::hog::Hog;
use chrono::Utc;
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
        let timestamp = Utc::now();

        let hog = Hog::new(
            client_request,
            Some(uuid.to_string()),
            Some(timestamp),
            None,
        );

        let payload = serde_json::to_vec(&hog)?;

        self.rabbit_channel
            .basic_publish(
                "",          // exchange, "" = default direct exchange
                "hog_queue", // routing key = queue name
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
}
