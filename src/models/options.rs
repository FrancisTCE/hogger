use bson::doc;
use mongodb::bson::{self, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsRequest {
    pub log_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub log_timestamp_start: Option<chrono::DateTime<chrono::Utc>>,
    pub log_timestamp_end: Option<chrono::DateTime<chrono::Utc>>,
    pub hog_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub hog_timestamp_start: Option<chrono::DateTime<chrono::Utc>>,
    pub hog_timestamp_end: Option<chrono::DateTime<chrono::Utc>>,
    pub log_level: Option<String>,
    pub log_message: Option<String>,
    pub log_data: Option<serde_json::Value>,
    pub log_data_field: Option<String>,
    pub log_data_value: Option<Option<serde_json::Value>>,
    pub log_data_fields: Option<Vec<String>>, // TODO: Implement this
    pub log_data_values: Option<Vec<Option<serde_json::Value>>>, // TODO: Implement this
    pub log_type: Option<String>,
    pub log_source: Option<String>,
    pub log_source_id: Option<String>,
    pub log_uuid: Option<String>,
    pub hog_uuid: Option<String>,
    pub hog_limit: Option<i64>,
    pub hog_parcial: Option<bool>, // TODO: Implement this, currently only partial on mensage
    pub hog_case_sensitive: Option<bool>, // TODO: Implement this
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
        if options.hog_parcial.unwrap_or(false) {
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
    if let Some(ref log_uuid) = options.log_uuid {
        filter.insert("log_uuid", log_uuid);
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
        let mut ts_filter = Document::new();

        if let Some(start) = options.log_timestamp_start {
            // Convert chrono::DateTime<Utc> to ISO 8601 string
            let start_str = start.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
            ts_filter.insert("$gte", start_str);
        }

        if let Some(end) = options.log_timestamp_end {
            let end_str = end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
            ts_filter.insert("$lte", end_str);
        }

        filter.insert("log_timestamp", ts_filter);
        println!("log_timestamp filter: {:?}", filter);
    } else if let Some(timestamp) = options.log_timestamp {
        let ts_str = timestamp.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        filter.insert("log_timestamp", ts_str);
    }

    if options.hog_timestamp.is_some()
        || options.hog_timestamp_start.is_some()
        || options.hog_timestamp_end.is_some()
    {
        let mut hog_ts_filter = Document::new();

        if let Some(start) = options.hog_timestamp_start {
            // Convert to ISO 8601 string with milliseconds + Z
            let start_str = start.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
            hog_ts_filter.insert("$gte", start_str);
        }

        if let Some(end) = options.hog_timestamp_end {
            let end_str = end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
            hog_ts_filter.insert("$lte", end_str);
        }

        if !hog_ts_filter.is_empty() {
            filter.insert("hog_timestamp", hog_ts_filter);
        }
    }

    if let (Some(field), Some(value)) = (&options.log_data_field, &options.log_data_value) {
        let key = format!("log_data.{}", field);
        match bson::to_bson(value) {
            Ok(bson_value) => {
                filter.insert(key, bson_value);
            }
            Err(e) => {
                eprintln!("Failed to convert log_data_value to BSON: {:?}", e);
            }
        }
    } else if let Some(ref field) = options.log_data_field {
        let key = format!("log_data.{}", field);
        filter.insert(key, doc! { "$exists": true });
    } else if let Some(ref _value) = options.log_data_value {
        eprintln!("Warning: Cannot search for value in arbitrary keys unless schema is known.");
    }

    if let Some(ref hog_timestamp) = options.hog_timestamp {
        let ts_str = hog_timestamp.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        filter.insert("hog_timestamp", ts_str);
    }

    filter
}
