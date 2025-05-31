use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRequest {
    pub log_timestamp: String,
    
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
}

