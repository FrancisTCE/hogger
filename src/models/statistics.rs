use bson::doc;
use mongodb::bson::{self, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HogStatistics {
    pub record_count: i64,
    pub log_level_count: i64,
    pub log_type_count: i64,
    pub log_source_count: i64,
    pub log_source_id_count: i64,
    pub log_data_count: i64,
    pub log_timestamp_count: i64,
    pub log_message_count: i64,
}

impl HogStatistics {
    #[allow(dead_code)]
    pub fn new() -> Self {
        HogStatistics {
            record_count: 0,
            log_level_count: 0,
            log_type_count: 0,
            log_source_count: 0,
            log_source_id_count: 0,
            log_data_count: 0,
            log_timestamp_count: 0,
            log_message_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn build_log_statistics_aggregation_pipeline() -> Vec<Document> {
        vec![doc! {
            "$group": {
                "_id": null,
                "record_count": { "$sum": 1 },
                "log_level_count": {
                    "$sum": { "$cond": [{ "$ifNull": ["$log_level", false] }, 1, 0] }
                },
                "log_type_count": {
                    "$sum": { "$cond": [{ "$ifNull": ["$log_type", false] }, 1, 0] }
                },
                "log_source_count": {
                    "$sum": { "$cond": [{ "$ifNull": ["$log_source", false] }, 1, 0] }
                },
                "log_source_id_count": {
                    "$sum": { "$cond": [{ "$ifNull": ["$log_source_id", false] }, 1, 0] }
                },
                "log_data_count": {
                    "$sum": { "$cond": [{ "$ifNull": ["$log_data", false] }, 1, 0] }
                },
                "log_timestamp_count": {
                    "$sum": { "$cond": [{ "$ifNull": ["$log_timestamp", false] }, 1, 0] }
                },
                "log_message_count": {
                    "$sum": { "$cond": [{ "$ifNull": ["$log_message", false] }, 1, 0] }
                }
            }
        }]
    }
}
