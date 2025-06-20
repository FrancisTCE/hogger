use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HogRequest {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorSchema {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub message: String,
    pub errors: Vec<ApiErrorSchema>,
}

#[allow(dead_code)]
pub async fn validate(req: serde_json::Value) -> Result<HogRequest, ErrorResponse> {
    let mut errors = Vec::new();

    let log_timestamp = match req.get("log_timestamp").and_then(|v| v.as_str()) {
        Some(ts) => ts.to_string(),
        None => {
            errors.push(ApiErrorSchema {
                field: "log_timestamp".to_string(),
                message: "log_timestamp is required".to_string(),
            });
            String::new()
        }
    };

    let log_message = match req.get("log_message").and_then(|v| v.as_str()) {
        Some(ts) => ts.to_string(),
        None => {
            errors.push(ApiErrorSchema {
                field: "log_message".to_string(),
                message: "log_message is required".to_string(),
            });
            String::new()
        }
    };

    let _payload = match req.get("log_data") {
        Some(val) => {
            if let Some(obj) = val.as_object() {
                if obj.is_empty() {
                    None
                } else {
                    Some(val.clone())
                }
            } else {
                errors.push(ApiErrorSchema {
                    field: "log_data".to_string(),
                    message: "log_data must be a valid JSON object".to_string(),
                });
                None
            }
        }
        None => None,
    };

    if errors.is_empty() {
    } else {
        return Err(ErrorResponse {
            status_code: 400,
            message: "Validation failed".to_string(),
            errors,
        });
    }

    let log_level = req
        .get("log_level")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let log_data = req.get("log_data").cloned();

    let log_type = req
        .get("log_type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let log_source = req
        .get("log_source")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let log_source_id = req
        .get("log_source_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(HogRequest {
        log_timestamp,
        log_level,
        log_message,
        log_data,
        log_type,
        log_source,
        log_source_id,
    })
}
