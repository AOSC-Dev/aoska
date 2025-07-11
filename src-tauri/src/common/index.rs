use crate::common::packages::{Category, PackageBrief};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryIndex {
    pub category: Category,
    pub packages: Vec<PackageBrief>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendIndex {
    pub date: DateTime<Utc>,
    pub packages: Vec<PackageBrief>,
}

// Serialized to aoska_index.json
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Index {
    pub version: u8,
    pub generated_at: DateTime<Utc>,
    pub packages: Vec<CategoryIndex>,
}
