use crate::{
    errors::{ApiErrorField, ValidationError},
    models::hog_client_schema::HogRequest,
};
use serde_json::Value;

#[allow(dead_code)]
pub async fn validate_hog_client_schema(req: Value) -> Result<HogRequest, ValidationError> {
    let mut errors = Vec::new();

    let log_timestamp = match req.get("log_timestamp").and_then(|v| v.as_str()) {
        Some(ts) => ts.to_string(),
        None => {
            errors.push(ApiErrorField {
                field: "log_timestamp".to_string(),
                message: "log_timestamp is required".to_string(),
            });
            String::new()
        }
    };

    let log_message = match req.get("log_message").and_then(|v| v.as_str()) {
        Some(msg) => msg.to_string(),
        None => {
            errors.push(ApiErrorField {
                field: "log_message".to_string(),
                message: "log_message is required".to_string(),
            });
            String::new()
        }
    };

    if !errors.is_empty() {
        return Err(ValidationError::new(errors));
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
