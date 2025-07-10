use serde::{Deserialize, Serialize};

use crate::common::utils::fetch_data;
use crate::common::packages::{Category, PackageBrief};

use std::path::PathBuf;
use reqwest::Client;
use chrono::{DateTime, Utc};
use anyhow::Result;

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

pub struct Index(pub Vec<PackageBrief>);

async fn fetch_index(client: &Client) -> Result<Index>;

async fn fetch_category_index(client: &Client, category: Category);

async fn fetch_recommend_index(client: &Client);
