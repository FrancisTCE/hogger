use std::collections::HashMap;

use bson::{Bson, doc};
use mongodb::bson::{self, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortType {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    None,
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsRequest {
    pub log_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub log_timestamp_start: Option<chrono::DateTime<chrono::Utc>>,
    pub log_timestamp_end: Option<chrono::DateTime<chrono::Utc>>,
    pub log_level: Option<String>,
    pub log_message: Option<String>,
    pub log_data: Option<serde_json::Value>,
    pub log_data_field: Option<String>,
    pub log_data_value: Option<Option<serde_json::Value>>,
    pub log_data_fields: Option<Vec<String>>, // TODO: Implement this
    pub log_data_values: Option<Option<serde_json::Value>>, // TODO: Implement this
    pub log_type: Option<String>,
    pub log_source: Option<String>,
    pub log_source_id: Option<String>,
    pub hog_uuid: Option<String>,
    pub hog_limit: Option<i64>,
    pub hog_sort: Option<SortType>,       // TODO: Implement this
    pub hog_partial: Option<bool>,        // TODO: Implement this, currently only partial on mensage
    pub hog_case_sensitive: Option<bool>, // TODO: Implement this
    pub hog_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub hog_timestamp_start: Option<chrono::DateTime<chrono::Utc>>,
    pub hog_timestamp_end: Option<chrono::DateTime<chrono::Utc>>,
}

#[allow(dead_code)]
pub fn build_filter(options: &OptionsRequest) -> Document {
    let mut filter = Document::new();

    if let Some(ref log_level) = options.log_level {
        filter.insert("log_level", log_level);
    }
    if let Some(ref log_source) = options.log_source {
        filter.insert("log_source", log_source);
    }
    if let Some(ref log_source_id) = options.log_source_id {
        filter.insert("log_source_id", log_source_id);
    }
    if let Some(ref log_message) = options.log_message {
        if options.hog_partial.unwrap_or(false) {
            filter.insert(
                "log_message",
                doc! {
                    "$regex": log_message,
                    "$options": "i",
                },
            );
        } else {
            filter.insert("log_message", log_message);
        }
    }
    if let Some(ref log_type) = options.log_type {
        filter.insert("log_type", log_type);
    }

    if let Some(ref hog_uuid) = options.hog_uuid {
        filter.insert("hog_uuid", hog_uuid);
    }

    if let Some(ref log_data) = options.log_data {
        match bson::to_bson(log_data) {
            Ok(bson_data) => {
                filter.insert("log_data", bson_data);
            }
            Err(_) => {}
        }
    }
    if options.log_timestamp_start.is_some() || options.log_timestamp_end.is_some() {
        let mut ts_filter: Document = Document::new();

        if let Some(start) = options.log_timestamp_start {
            ts_filter.insert("$gte", bson::DateTime::from_chrono(start));
        }

        if let Some(end) = options.log_timestamp_end {
            ts_filter.insert("$lte", bson::DateTime::from_chrono(end));
        }
        if ts_filter.is_empty() {
            return filter;
        }
        filter.insert("log_timestamp", ts_filter);
    } else if let Some(timestamp) = options.log_timestamp {
        filter.insert("log_timestamp", bson::DateTime::from_chrono(timestamp));
    }

    if options.hog_timestamp.is_some()
        || options.hog_timestamp_start.is_some()
        || options.hog_timestamp_end.is_some()
    {
        let mut hog_ts_filter = Document::new();

        if let Some(start) = options.hog_timestamp_start {
            hog_ts_filter.insert("$gte", bson::DateTime::from_chrono(start));
        }

        if let Some(end) = options.hog_timestamp_end {
            hog_ts_filter.insert("$lte", bson::DateTime::from_chrono(end));
        }

        if !hog_ts_filter.is_empty() {
            filter.insert("hog_timestamp", hog_ts_filter);
        }
    }

    if let Some(hog_timestamp) = options.hog_timestamp {
        filter.insert("hog_timestamp", bson::DateTime::from_chrono(hog_timestamp));
    }

    if let Some(log_data_value) = &options.log_data_value {
        if let Some(field) = &options.log_data_field {
            let key = format!("log_data.{}", field);
            if let Ok(bson_value) = bson::to_bson(log_data_value) {
                filter.insert(key, bson_value);
            }
        } else {
            //needs to run aggregations
        }
    } else if let Some(field) = &options.log_data_field {
        let key = format!("log_data.{}", field);
        filter.insert(key, doc! { "$exists": true });
    }

    if let Some(Some(log_data_values)) = &options.log_data_values {
        let mut flat_doc = Document::new();
        if let Some(map) = log_data_values.as_object() {
            for (k, v) in map {
                let prefixed = format!("log_data.{}", k);
                flat_doc.insert(prefixed.clone(), Bson::from(bson::to_bson(v).unwrap()));
            }
        }
        filter.extend(flat_doc);
    }
    filter
}

#[allow(dead_code)]
pub fn build_log_data_value_aggregation_pipeline(
    value: &serde_json::Value,
    options: &OptionsRequest,
) -> Vec<Document> {
    let bson_value = match bson::to_bson(value) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let mut pipeline = vec![
        doc! {
            "$match": {
                "log_data": { "$exists": true },
                "log_data": { "$type": "object" }
            }
        },
        doc! {
            "$project": {
                "log_data": 1,
                "_id": 1,
                "log_level": 1,
                "log_message": 1,
                "log_type": 1,
                "log_source": 1,
                "log_source_id": 1,
                "hog_uuid": 1,
                "hog_timestamp": 1,
                "log_timestamp": 1
            }
        },
        doc! {
            "$match": {
                "$expr": {
                    "$in": [
                        bson_value,
                        {
                            "$map": {
                                "input": { "$objectToArray": "$log_data" },
                                "as": "entry",
                                "in": { "$ifNull": ["$$entry.v", ""] }
                            }
                        }
                    ]
                }
            }
        },
    ];

    let mut new_options = options.clone();
    new_options.log_data_value = None;
    let extra_match = build_filter(&new_options);

    if !extra_match.is_empty() {
        pipeline.push(doc! { "$match": extra_match });
    }

    pipeline
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
pub async fn validate_options(req: serde_json::Value) -> Result<OptionsRequest, ErrorResponse> {
    let mut errors = Vec::new();

    // Example: extract fields as Option<T>
    let log_level = req
        .get("log_level")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref lvl) = log_level {
        if lvl.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_level".to_string(),
                message: "log_level cannot be empty".to_string(),
            });
        }
    }

    let log_message = req
        .get("log_message")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref msg) = log_message {
        if msg.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_message".to_string(),
                message: "log_message cannot be empty".to_string(),
            });
        }
    }

    let log_data_values = req.get("log_data_values").cloned();
    if let Some(ref log_data) = log_data_values {
        if !log_data.is_object() {
            errors.push(ApiErrorSchema {
                field: "log_data".to_string(),
                message: "log_data must be a valid JSON object".to_string(),
            });
        }
    }

    let log_type = req
        .get("log_type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref t) = log_type {
        if t.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_type".to_string(),
                message: "log_type cannot be empty".to_string(),
            });
        }
    }

    let log_source = req
        .get("log_source")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref src) = log_source {
        if src.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_source".to_string(),
                message: "log_source cannot be empty".to_string(),
            });
        }
    }

    let log_source_id = req
        .get("log_source_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref src_id) = log_source_id {
        if src_id.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_source_id".to_string(),
                message: "log_source_id cannot be empty".to_string(),
            });
        }
    }

    let log_timestamp = req
        .get("log_timestamp")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    if let Some(ts) = log_timestamp {
        if ts.timestamp() < 0 {
            errors.push(ApiErrorSchema {
                field: "log_timestamp".to_string(),
                message: "log_timestamp cannot be before epoch".to_string(),
            });
        }
    }
    let log_timestamp_start = req
        .get("log_timestamp_start")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    if let Some(ts) = log_timestamp_start {
        if ts.timestamp() < 0 {
            errors.push(ApiErrorSchema {
                field: "log_timestamp_start".to_string(),
                message: "log_timestamp_start cannot be before epoch".to_string(),
            });
        }
    }
    let log_timestamp_end = req
        .get("log_timestamp_end")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    if let Some(ts) = log_timestamp_end {
        if ts.timestamp() < 0 {
            errors.push(ApiErrorSchema {
                field: "log_timestamp_end".to_string(),
                message: "log_timestamp_end cannot be before epoch".to_string(),
            });
        }
    }

    let hog_uuid = req
        .get("hog_uuid")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref uuid) = hog_uuid {
        if uuid.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "hog_uuid".to_string(),
                message: "hog_uuid cannot be empty".to_string(),
            });
        }
    }
    let hog_limit = req.get("hog_limit").and_then(|v| v.as_i64());
    if let Some(limit) = hog_limit {
        if limit <= 0 {
            errors.push(ApiErrorSchema {
                field: "hog_limit".to_string(),
                message: "hog_limit must be greater than zero".to_string(),
            });
        }
    }
    let hog_partial = req.get("hog_partial").and_then(|v| v.as_bool());
    if let Some(partial) = hog_partial {
        if !partial {
            errors.push(ApiErrorSchema {
                field: "hog_partial".to_string(),
                message: "hog_partial must be true for this operation".to_string(),
            });
        }
    }
    let hog_case_sensitive = req.get("hog_case_sensitive").and_then(|v| v.as_bool());
    if let Some(case_sensitive) = hog_case_sensitive {
        if !case_sensitive {
            errors.push(ApiErrorSchema {
                field: "hog_case_sensitive".to_string(),
                message: "hog_case_sensitive must be true for this operation".to_string(),
            });
        }
    }
    let hog_sort = req.get("hog_sort").and_then(|v| v.as_str()).map(|s| {
        Some(match s {
            "ascending" => SortType::Ascending,
            "descending" => SortType::Descending,
            _ => {
                errors.push(ApiErrorSchema {
                    field: "hog_sort".to_string(),
                    message: "hog_sort must be 'ascending' or 'descending'".to_string(),
                });
                return None;
            }
        })
    });

    let hog_sort_field = req
        .get("hog_sort_field")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref sort_field) = hog_sort_field {
        if sort_field.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "hog_sort_field".to_string(),
                message: "hog_sort_field cannot be empty".to_string(),
            });
        }
    }
    let hog_fields = req.get("hog_fields").and_then(|v| v.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<String>>()
    });
    if let Some(ref fields) = hog_fields {
        if fields.is_empty() {
            errors.push(ApiErrorSchema {
                field: "hog_fields".to_string(),
                message: "hog_fields cannot be empty".to_string(),
            });
        }
    }

    let hog_timestamp = req
        .get("hog_timestamp")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    if let Some(ts) = hog_timestamp {
        if ts.timestamp() < 0 {
            errors.push(ApiErrorSchema {
                field: "hog_timestamp".to_string(),
                message: "hog_timestamp cannot be before epoch".to_string(),
            });
        }
    }
    let hog_timestamp_start = req
        .get("hog_timestamp_start")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    if let Some(ts) = hog_timestamp_start {
        if ts.timestamp() < 0 {
            errors.push(ApiErrorSchema {
                field: "hog_timestamp_start".to_string(),
                message: "hog_timestamp_start cannot be before epoch".to_string(),
            });
        }
    }
    let hog_timestamp_end = req
        .get("hog_timestamp_end")
        .and_then(|v| v.as_str())
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    if let Some(ts) = hog_timestamp_end {
        if ts.timestamp() < 0 {
            errors.push(ApiErrorSchema {
                field: "hog_timestamp_end".to_string(),
                message: "hog_timestamp_end cannot be before epoch".to_string(),
            });
        }
    }

    let log_data = req.get("log_data").and_then(|v| v.as_object()).cloned();
    if let Some(ref data) = log_data {
        if data.is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_data".to_string(),
                message: "log_data cannot be empty".to_string(),
            });
        }
    }
    let log_data_field = req
        .get("log_data_field")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if let Some(ref field) = log_data_field {
        if field.trim().is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_data_field".to_string(),
                message: "log_data_field cannot be empty".to_string(),
            });
        }
    }
    let log_data_value = req
        .get("log_data_value")
        .and_then(|v| v.as_object())
        .cloned();
    if let Some(ref value) = log_data_value {
        if value.is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_data_value".to_string(),
                message: "log_data_value cannot be empty".to_string(),
            });
        }
    }
    let log_data_fields = req
        .get("log_data_fields")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        });
    if let Some(ref fields) = log_data_fields {
        if fields.is_empty() {
            errors.push(ApiErrorSchema {
                field: "log_data_fields".to_string(),
                message: "log_data_fields cannot be empty".to_string(),
            });
        }
    }

    if !errors.is_empty() {
        return Err(ErrorResponse {
            status_code: 400,
            message: "Validation errors occurred".to_string(),
            errors,
        });
    }

    // Build OptionsRequest from validated fields
    let options = OptionsRequest {
        log_level,
        log_message,
        log_data_values: log_data_values.map(Some), // or adjust as needed for your struct
        log_type,
        log_source,
        log_source_id,
        log_timestamp,
        log_timestamp_start,
        log_timestamp_end,
        log_data: log_data.map(serde_json::Value::Object),
        log_data_field,
        log_data_value: log_data_value.map(|v| Some(serde_json::Value::Object(v))),
        log_data_fields,
        hog_uuid,
        hog_limit,
        hog_partial,
        hog_sort: hog_sort.flatten(),
        hog_case_sensitive,
        hog_timestamp,
        hog_timestamp_start,
        hog_timestamp_end,
    };

    Ok(options)
}
