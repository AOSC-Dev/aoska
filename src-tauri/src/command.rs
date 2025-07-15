use crate::common::{
    config::{ASM_INDEX_PATH, ASM_RECOMMEND_INDEX_PATH},
    index::{CategoryIndex, Index, RecommendIndex},
    oma::{
        check_security_upgradable, check_security_upgradable_detail, check_upgradable,
        check_upgradable_detail,
    },
    packages::{Category, PackageDetail},
    utils::fetch_data,
};

use anyhow::Result;
use oma_pm::apt::{AptConfig, OmaApt, OmaAptArgs};

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
    fetch_data(&app.client, ASM_RECOMMEND_INDEX_PATH)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_by_category(
    app: tauri::State<'_, AppState>,
    category: String,
) -> Result<CategoryIndex, String> {
    let index_data: Index = fetch_data(&app.client, ASM_INDEX_PATH)
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
    fetch_data(&app.client, ASM_INDEX_PATH)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_detail(
    app: tauri::State<'_, AppState>,
    pkg_name: String,
) -> Result<PackageDetail, String> {
    let path = format!("packages/{pkg_name}/meta.json");
    fetch_data(&app.client, &path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_update() -> Result<usize, String> {
    let apt = OmaApt::new(
        vec![],
        OmaAptArgs::builder().build(),
        false,
        AptConfig::new(),
    )
    .map_err(|e| e.to_string())?;
    let count = check_upgradable(&apt).await.map_err(|e| e.to_string())?;
    Ok(count)
}

#[tauri::command]
pub async fn fetch_security_update() -> Result<usize, String> {
    let apt = OmaApt::new(
        vec![],
        OmaAptArgs::builder().build(),
        false,
        AptConfig::new(),
    )
    .map_err(|e| e.to_string())?;
    let count = check_security_upgradable(&apt)
        .await
        .map_err(|e| e.to_string())?;
    Ok(count)
}
