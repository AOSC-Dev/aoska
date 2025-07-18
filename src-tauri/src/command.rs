use crate::common::{
    config::{ASM_INDEX_PATH, ASM_RECOMMEND_INDEX_PATH},
    index::{CategoryIndex, Index, RecommendIndex},
    oma::{check_tum_upgradable, check_upgradable, check_upgradable_count, TumUpdateInfo},
    packages::{Category, PackageDetail},
    utils::fetch_data,
};

use anyhow::Result;
use oma_pm::apt::{AptConfig, OmaApt, OmaAptArgs, OmaOperation};

pub struct AppState {
    client: reqwest::Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("aoska/1.0")
                .build()
                .expect("Reqwest Client"),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
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
pub async fn fetch_update_count(_app: tauri::State<'_, AppState>) -> Result<usize, String> {
    tokio::task::spawn_blocking(move || {
        let apt = OmaApt::new(
            vec![],
            OmaAptArgs::builder().build(),
            false,
            AptConfig::new(),
        )
        .map_err(|e| e.to_string())?;

        // OmaApt type from the oma-pm contains fields that are not Send or Sync
        // which means they cannot be safely shared between threads 
        // Use tokio runtime to run the async function in blocking context
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            check_upgradable_count(&apt)
                .await
                .map_err(|e| e.to_string())
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn fetch_update_detail(_app: tauri::State<'_, AppState>) -> Result<OmaOperation, String> {
    tokio::task::spawn_blocking(move || {
        let apt = OmaApt::new(
            vec![],
            OmaAptArgs::builder().build(),
            false,
            AptConfig::new(),
        )
        .map_err(|e| e.to_string())?;

        // OmaApt type from the oma-pm contains fields that are not Send or Sync
        // which means they cannot be safely shared between threads 
        // Use tokio runtime to run the async function in blocking context
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            check_upgradable(&apt)
                .await
                .map_err(|e| e.to_string())
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn fetch_tum_update(
    _app: tauri::State<'_, AppState>,
) -> Result<Vec<TumUpdateInfo>, String> {
    tokio::task::spawn_blocking(move || {
        let apt = OmaApt::new(
            vec![],
            OmaAptArgs::builder().build(),
            false,
            AptConfig::new(),
        )
        .map_err(|e| e.to_string())?;

        // OmaApt type from the oma-pm contains fields that are not Send or Sync
        // which means they cannot be safely shared between threads 
        // Use tokio runtime to run the async function in blocking context
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            check_tum_upgradable(&apt)
                .await
                .map_err(|e| e.to_string())
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
