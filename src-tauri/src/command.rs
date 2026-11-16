use crate::common::{
    amo::AmoBackend,
    amo_types::{
        PmCapabilities, PmOperationLogs, PmOperationStart, PmOperationStatus, PmPackageList,
        PmPlan, PmUpdateSummary, TumUpdateInfo,
    },
    config::{ASM_ENDPOINT, ASM_INDEX_PATH, ASM_RECOMMEND_INDEX_PATH},
    index::{CategoryIndex, Index, RecommendIndex},
    packages::{Category, PackageDetail},
    utils::fetch_data,
};
use anyhow::Result;
use serde_json::Value;

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
                e.file_type().is_file()
                    && if let Some(ext) = e.path().extension() {
                        matches!(
                            ext.to_string_lossy().to_lowercase().as_str(),
                            "json" | "png" | "jpg" | "jpeg"
                        )
                    } else {
                        false
                    }
            })
        {
            let rel = entry.path().strip_prefix("mock_data").unwrap();
            let url = format!("/{}", rel.to_string_lossy().replace('\\', "/"));

            server.mock(|when, then| {
                when.method(GET).path(url);

                let content_type = match entry
                    .path()
                    .extension()
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
pub async fn get_endpoint_base_url(app: tauri::State<'_, AppState>) -> Result<String, String> {
    Ok(app.base_url.clone())
}

#[tauri::command]
pub async fn pm_capabilities(
    backend: tauri::State<'_, AmoBackend>,
) -> Result<PmCapabilities, String> {
    backend
        .capabilities()
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_list_updates(
    backend: tauri::State<'_, AmoBackend>,
) -> Result<PmPackageList, String> {
    backend
        .list_updates()
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_update_summary(
    backend: tauri::State<'_, AmoBackend>,
) -> Result<PmUpdateSummary, String> {
    backend
        .update_summary()
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_tum_updates(
    backend: tauri::State<'_, AmoBackend>,
) -> Result<Vec<TumUpdateInfo>, String> {
    backend
        .tum_updates()
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_package_state(
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmPackageList, String> {
    backend
        .package_state(&packages)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub fn pm_list_installed() -> Result<PmPackageList, String> {
    Err("UNSUPPORTED_CAPABILITY: amo does not expose a complete installed-package inventory".into())
}

#[tauri::command]
pub async fn pm_plan_update(
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmPlan, String> {
    backend
        .plan("update", &packages)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_plan_install(
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmPlan, String> {
    backend
        .plan("install", &packages)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_plan_remove(
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmPlan, String> {
    backend
        .plan("remove", &packages)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_start_update(
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmOperationStart, String> {
    backend
        .start("update", &packages)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_start_install(
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmOperationStart, String> {
    backend
        .start("install", &packages)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_start_remove(
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmOperationStart, String> {
    backend
        .start("remove", &packages)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn pm_start_refresh(
    backend: tauri::State<'_, AmoBackend>,
) -> Result<PmOperationStart, String> {
    backend
        .start("refresh", &[])
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub fn pm_operation_status(
    backend: tauri::State<'_, AmoBackend>,
    id: String,
) -> Result<PmOperationStatus, String> {
    backend.status(&id).map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub fn pm_operation_result(
    backend: tauri::State<'_, AmoBackend>,
    id: String,
) -> Result<PmOperationStatus, String> {
    backend.status(&id).map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub fn pm_operation_logs(
    backend: tauri::State<'_, AmoBackend>,
    id: String,
) -> Result<PmOperationLogs, String> {
    backend.logs(&id).map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub fn pm_cancel_operation(id: String) -> Result<Value, String> {
    Err(format!(
        "UNSUPPORTED_CAPABILITY: amo cannot cancel operation {id}"
    ))
}
