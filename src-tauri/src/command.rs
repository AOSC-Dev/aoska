use crate::common::{
    index::{CategoryIndex, Index, RecommendIndex},
    packages::{Category, PackageDetail},
    utils::fetch_data,
};

use anyhow::Result;
use tauri::App;

pub struct AppState {
    client: reqwest::Client,
}

impl AppState {
    fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("aosks/1.0")
                .build()
                .expect("Reqwest Client"),
        }
    }
}

#[tauri::command]
pub async fn fetch_recommend(app: tauri::State<'_, AppState>) -> Result<RecommendIndex, String> {
    fetch_data(&app.client, "recommend_index.json")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_by_category(
    app: tauri::State<'_, AppState>,
    category: String,
) -> Result<CategoryIndex, String> {
    let index_data: Index = fetch_data(&app.client, "aoska_index.json")
        .await
        .map_err(|e| e.to_string())?;
    let cat = category
        .parse::<Category>()
        .map_err(|_| format!("Unknown Category: {category}"))?;

    index_data
        .packages
        .into_iter()
        .find(|ci| ci.category == cat)
        .ok_or_else(|| format!("Category {category} not found"))
}

#[tauri::command]
pub async fn fetch_index(app: tauri::State<'_, AppState>) -> Result<Index, String> {
    fetch_data(&app.client, "aoska_index.json")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_detail(
    app: tauri::State<'_, AppState>,
    pkg_name: String,
) -> Result<PackageDetail, String> {
    let path = format!("pkgs/{pkg_name}");
    fetch_data(&app.client, &path)
        .await
        .map_err(|e| e.to_string())
}
