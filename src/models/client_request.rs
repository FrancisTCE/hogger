use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRequest {
    pub log_timestamp: chrono::DateTime<chrono::Utc>,
    pub log_level: Option<String>,
    pub log_message: String,
    pub log_data: Option<serde_json::Value>, 
    pub log_type: Option<String>,
    pub log_source: Option<String>,
    pub log_source_id: Option<String>,
}
