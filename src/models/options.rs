use std::collections::HashMap;

use bson::{doc, Bson};
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
    pub hog_partial: Option<bool>, // TODO: Implement this, currently only partial on mensage
    pub hog_case_sensitive: Option<bool>, // TODO: Implement this
    pub hog_sort: Option<SortType>, // TODO: Implement this 
    pub hog_sort_field: Option<String>, // TODO: Implement this
    pub hog_fields: Option<Vec<String>>, // TODO: Implement this
    pub hog_values: Option<Vec<Option<Value>>>, // TODO: Implement this
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
            flat_doc.insert(
                prefixed,
                Bson::from(bson::to_bson(v).unwrap())
            );
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

fn flatten_json(prefix: Option<String>, value: &Value, doc: &mut Document) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let new_prefix = match &prefix {
                    Some(p) => format!("{}.{}", p, k),
                    None => k.clone(),
                };
                flatten_json(Some(new_prefix), v, doc);
            }
        }
        _ => {
            if let Some(key) = prefix {
                // Convert serde_json::Value to Bson
                let bson_value = bson::to_bson(value).unwrap_or(Bson::Null);
                doc.insert(key, bson_value);
            }
        }
    }
}