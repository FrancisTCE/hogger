
use crate::models::hog::Hog;
use crate::models::client_request::ClientRequest;
use futures::StreamExt;
use mongodb::{Collection, Database, bson::doc};
use uuid::Uuid;
use chrono::Utc;



pub struct HogService {
    collection: Collection<Hog>,
}

impl HogService {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<Hog>("hog");
        HogService { collection }
    }

    pub async fn create_hog(&self, client_request:ClientRequest) -> mongodb::error::Result<Hog> {
        let uuid = Uuid::new_v4();
        let timestamp = Utc::now();
        
        let hog = Hog::new(
            client_request,
            Some(uuid.to_string()),
            Some(timestamp),
            None,
        );
        let insert_result = self.collection.insert_one(hog.clone()).await?;
        let id = insert_result.inserted_id;
        let mut hog = hog;
        hog.id = match id {
            mongodb::bson::Bson::ObjectId(oid) => Some(oid),
            _ => None,
        };
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
