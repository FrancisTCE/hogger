use bson::doc;
use mongodb::bson::{self};
use serde::{Deserialize, Serialize};

use super::options::OptionsRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortType {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCallbackOptions {
    pub webhook: Option<String>,
    pub headers: Option<bson::Document>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFieldOptions {
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
    pub advanced_options: Option<Vec<AdvancedFieldOptions>>,
    pub simple_options: Option<OptionsRequest>
}