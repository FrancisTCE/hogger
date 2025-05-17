use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsRequest {
    pub search_type: String,
    pub log_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub log_timestamp_start: Option<chrono::DateTime<chrono::Utc>>,
    pub log_timestamp_end: Option<chrono::DateTime<chrono::Utc>>,
    pub log_level: Option<String>,
    pub log_message: Option<String>,
    pub log_data: Option<serde_json::Value>, 
    pub log_type: Option<String>,
    pub log_source: Option<String>,
    pub log_source_id: Option<String>,
    pub log_uuid: Option<String>,
    pub hog_uuid: Option<String>,
    pub hog_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}
