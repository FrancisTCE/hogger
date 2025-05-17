
use crate::models::hog::Hog;
use crate::models::client_request::ClientRequest;
use futures::StreamExt;
use mongodb::{Collection, Database, bson::doc};
use uuid::Uuid;
use chrono::Utc;
use lapin::{Channel, BasicProperties};


pub struct HogService {
    collection: Collection<Hog>,
    rabbit_channel: Channel,
}

impl HogService {
    pub fn new(db: &Database, rabbit_channel: Channel) -> Self {
        let collection = db.collection::<Hog>("hog");
        HogService { collection, rabbit_channel }
    }

    pub async fn create_hog(&self, client_request:ClientRequest) -> anyhow::Result<Hog> {
        let uuid = Uuid::new_v4();
        let timestamp = Utc::now();
        
        let hog = Hog::new(
            client_request,
            Some(uuid.to_string()),
            Some(timestamp),
            None,
        );
        // Serialize hog to JSON string
        let payload = serde_json::to_vec(&hog)?;

        // Publish to RabbitMQ queue named "hog_queue" (change if needed)
        self.rabbit_channel
            .basic_publish(
                "",             // exchange, "" = default direct exchange
                "hog_queue",    // routing key = queue name
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
}
