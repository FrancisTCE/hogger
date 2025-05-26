use bson::doc;
use mongodb::bson::{self, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortType {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOptions {
    pub field: Option<String>,
    pub value: Option<serde_json::Value>,
    pub limit: Option<i64>,
    pub partial: Option<bool>,
    pub case_sensitive: Option<bool>,
    pub sort: Option<SortType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedOptionsRequest {
    pub pagination_token: Option<String>,
    pub filters: Option<Vec<FieldOptions>>,
}