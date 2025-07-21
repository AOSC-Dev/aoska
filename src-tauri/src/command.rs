use crate::common::{
    config::{ASM_ENDPOINT, ASM_INDEX_PATH, ASM_RECOMMEND_INDEX_PATH},
    index::{CategoryIndex, Index, RecommendIndex},
    oma::{check_tum_upgradable, check_upgradable, check_upgradable_count, TumUpdateInfo},
    packages::{Category, PackageDetail},
    utils::fetch_data,
};

use anyhow::Result;
use oma_pm::apt::{AptConfig, OmaApt, OmaAptArgs, OmaOperation};

#[cfg(debug_assertions)]
use httpmock::prelude::*;

pub struct AppState {
    client: reqwest::Client,
    base_url: String,
    #[cfg(debug_assertions)]
    _mock_server: MockServer,
}

#[allow(unreachable_code)]
impl AppState {
    pub fn prod() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("aoska/1.0")
                .build()
                .expect("Reqwest Client"),
            base_url: ASM_ENDPOINT.to_string(),

            #[cfg(debug_assertions)]
            _mock_server: unreachable!(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn dev() -> Self {
        use httpmock::prelude::*;
        use walkdir::WalkDir;

        let server = MockServer::start();
        for entry in WalkDir::new("mock_data")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().is_file() && 
                if let Some(ext) = e.path().extension() {
                    matches!(ext.to_string_lossy().to_lowercase().as_str(), "json" | "png" | "jpg" | "jpeg")
                } else {
                    false
                }
            })
        {
            let rel = entry.path().strip_prefix("mock_data").unwrap();
            let url = format!("/{}", rel.to_string_lossy().replace('\\', "/"));

            server.mock(|when, then| {
                when.method(GET).path(url);
                
                let content_type = match entry.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.to_lowercase())
                    .as_deref()
                {
                    Some("json") => "application/json",
                    Some("png") => "image/png",
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    _ => "application/octet-stream",
                };
                
                then.status(200)
                    .header("content-type", content_type)
                    .body_from_file(entry.path().to_string_lossy().to_string());
            });
        }

        Self {
            client: reqwest::Client::builder()
                .user_agent("aoska/1.0")
                .build()
                .expect("Reqwest Client"),
            base_url: server.base_url(),
            _mock_server: server,
        }
    }
}

impl Default for AppState {
    #[cfg(not(debug_assertions))]
    fn default() -> Self {
        Self::prod()
    }

    #[cfg(debug_assertions)]
    fn default() -> Self {
        Self::dev()
    }
}

#[tauri::command]
pub async fn fetch_recommend(app: tauri::State<'_, AppState>) -> Result<RecommendIndex, String> {
    fetch_data(&app.client, &app.base_url, ASM_RECOMMEND_INDEX_PATH)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_by_category(
    app: tauri::State<'_, AppState>,
    category: String,
) -> Result<CategoryIndex, String> {
    let index_data: Index = fetch_data(&app.client, &app.base_url, ASM_INDEX_PATH)
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
    fetch_data(&app.client, &app.base_url, ASM_INDEX_PATH)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_detail(
    app: tauri::State<'_, AppState>,
    pkg_name: String,
) -> Result<PackageDetail, String> {
    let path = format!("packages/{pkg_name}/meta.json");
    fetch_data(&app.client, &app.base_url, &path)
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
        rt.block_on(async { check_upgradable(&apt).await.map_err(|e| e.to_string()) })
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
        rt.block_on(async { check_tum_upgradable(&apt).await.map_err(|e| e.to_string()) })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_endpoint_base_url(app: tauri::State<'_, AppState>) -> Result<String> {
    Ok(app.base_url.clone())
}
