use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

use super::client_request::ClientRequest;

 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hog {
    #[serde(flatten)]
    pub client_request: ClientRequest,
    pub hog_uuid: Option<String>,
    pub hog_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

#[allow(dead_code)]
impl Hog {
    pub fn new(
        client_request: ClientRequest,
        hog_uuid: Option<String>,
        hog_timestamp: Option<DateTime<Utc>>,
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
