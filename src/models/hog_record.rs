use bson::DateTime as BsonDateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HogRecord {
    pub log_timestamp: BsonDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,

    pub log_message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_data: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_source_id: Option<String>,

    pub hog_uuid: Option<String>,
    pub hog_timestamp: Option<BsonDateTime>,

    pub created_at: Option<BsonDateTime>,

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}
