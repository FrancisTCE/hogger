use bson::doc;
use serde::{Deserialize, Serialize};

use super::hog::Hog;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedOptionsResponse {
    pub pagination_token: Option<String>,
    pub document_count: i64,
    pub hogs: Vec<Hog>,
}