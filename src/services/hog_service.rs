use crate::models::client_request::ClientRequest;
use crate::models::hog::Hog;
use crate::models::options::{self, OptionsRequest};
use crate::utils::utils;
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
        let limit = options.hog_limit.unwrap_or(1000);
        let mut cursor = self.collection.find(filter).limit(limit).with_options(None).await?;
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
