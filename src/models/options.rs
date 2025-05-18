
use serde::{Deserialize, Serialize};
use mongodb::bson::{self, Document};



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
    pub log_type: Option<String>,
    pub log_source: Option<String>,
    pub log_source_id: Option<String>,
    pub log_uuid: Option<String>,
    pub hog_uuid: Option<String>,
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
        filter.insert("log_message", log_message);
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
            ts_filter.insert("$gte", bson::DateTime::from_millis(start.timestamp_millis()));
        }
        if let Some(end) = options.log_timestamp_end {
            ts_filter.insert("$lte", bson::DateTime::from_millis(end.timestamp_millis()));
        }
        filter.insert("log_timestamp", ts_filter);
        println!("log_timestamp filter: {:?}", filter);
    } else if let Some(timestamp) = options.log_timestamp {
        filter.insert("log_timestamp", bson::DateTime::from_millis(timestamp.timestamp_millis()));
    }


    if options.hog_timestamp.is_some() || options.hog_timestamp_start.is_some() || options.hog_timestamp_end.is_some() {
        let mut hog_ts_filter = Document::new();
        if let Some(start) = options.hog_timestamp_start {
            hog_ts_filter.insert("$gte", bson::DateTime::from_millis(start.timestamp_millis()));
        }
        if let Some(end) = options.hog_timestamp_end {
            hog_ts_filter.insert("$lte", bson::DateTime::from_millis(end.timestamp_millis()));
        }
        if !hog_ts_filter.is_empty() {
            filter.insert("hog_timestamp", hog_ts_filter);
        }
    }

    if let Some(hog_ts) = options.hog_timestamp {
        filter.insert("hog_timestamp", bson::DateTime::from_millis(hog_ts.timestamp_millis()));
    }

    filter
}

