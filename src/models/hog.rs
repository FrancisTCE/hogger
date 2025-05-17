use serde::{Deserialize, Serialize};
use crate::models::client_request::ClientRequest;
use mongodb::bson::oid::ObjectId;

 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hog {
    #[serde(flatten)]
    pub client_request: ClientRequest,
    pub hog_uuid: Option<String>,
    pub hog_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

impl Hog {
    pub fn new(
        client_request: ClientRequest,
        hog_uuid: Option<String>,
        hog_timestamp: Option<chrono::DateTime<chrono::Utc>>,
        id: Option<ObjectId>,
    ) -> Self {
        Hog {
            client_request,
            hog_uuid,
            hog_timestamp,
            id,
        }
    }
}
