use crate::common::{
    amo::AmoBackend,
    amo_types::{
        PmCapabilities, PmOperationLogs, PmOperationStart, PmOperationStatus, PmPackageList,
        PmPlan, PmUpdateSummary, TumUpdateInfo,
    },
    catalog::{installed_entries, preferred_desktop_id, unique_entries},
    config::{ASM_ENDPOINT, ASM_INDEX_PATH, ASM_RECOMMEND_INDEX_PATH},
    desktop::{self, DesktopApplication},
    index::{CategoryIndex, Index, RecommendIndex},
    packages::{Category, PackageDetail},
    utils::fetch_data,
};
use anyhow::{bail, Context, Result};
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

impl AppState {
    async fn catalog_entries(&self) -> Result<Vec<crate::common::packages::PackageBrief>> {
        let index: Index = fetch_data(&self.client, &self.base_url, ASM_INDEX_PATH).await?;
        Ok(unique_entries(index))
    }

    async fn desktop_application(
        &self,
        name: &str,
        index_id: Option<&str>,
    ) -> Result<Option<DesktopApplication>> {
        if index_id.is_some() {
            return desktop::resolve(name, index_id);
        }
        let detail: PackageDetail = fetch_data(
            &self.client,
            &self.base_url,
            &format!("packages/{name}/meta.json"),
        )
        .await?;
        desktop::resolve(
            name,
            preferred_desktop_id(index_id, detail.package_info.desktop_id.as_deref()),
        )
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
    let mut capabilities = backend
        .capabilities()
        .await
        .map_err(|error| format!("{error:#}"))?;
    capabilities
        .capabilities
        .push("query.catalog-installed.v1".into());
    if cfg!(target_os = "linux") {
        capabilities.capabilities.push("desktop.launch.v1".into());
    }
    Ok(capabilities)
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
    app: tauri::State<'_, AppState>,
    backend: tauri::State<'_, AmoBackend>,
    packages: Vec<String>,
) -> Result<PmPackageList, String> {
    let mut result = backend
        .package_state(&packages)
        .await
        .map_err(|error| format!("{error:#}"))?;
    if let Ok(entries) = app.catalog_entries().await {
        for package in &mut result.packages {
            if package.installed {
                if let Some(entry) = entries.iter().find(|entry| entry.name == package.name) {
                    match app
                        .desktop_application(&entry.name, entry.desktop_id.as_deref())
                        .await
                    {
                        Ok(application) => package.openable = application.is_some(),
                        Err(error) => eprintln!("Desktop lookup for {}: {error:#}", entry.name),
                    }
                }
            }
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn pm_list_catalog_installed(
    app: tauri::State<'_, AppState>,
    backend: tauri::State<'_, AmoBackend>,
) -> Result<Value, String> {
    async fn list(app: &AppState, backend: &AmoBackend) -> Result<Value> {
        let entries = app
            .catalog_entries()
            .await
            .context("Cannot load the application catalog")?;
        if entries.is_empty() {
            return Ok(serde_json::json!({"packages":[]}));
        }
        let names = entries
            .iter()
            .map(|entry| entry.name.clone())
            .collect::<Vec<_>>();
        let states = backend.package_state(&names).await?;
        let mut applications = installed_entries(entries, states.packages, &app.base_url)?;
        for application in &mut applications {
            match app
                .desktop_application(&application.catalog_name, application.desktop_id.as_deref())
                .await
            {
                Ok(Some(desktop)) => {
                    application.package.openable = true;
                    application.package.display_name = desktop.name;
                    application.desktop_id = Some(desktop.id);
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("Desktop lookup for {}: {error:#}", application.catalog_name)
                }
            }
        }
        Ok(serde_json::json!({"packages":applications}))
    }
    list(&app, &backend)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn app_launch(
    app: tauri::State<'_, AppState>,
    backend: tauri::State<'_, AmoBackend>,
    catalog_name: String,
) -> Result<(), String> {
    async fn launch(app: &AppState, backend: &AmoBackend, name: String) -> Result<()> {
        let entry = app
            .catalog_entries()
            .await?
            .into_iter()
            .find(|entry| entry.name == name)
            .context("Application is not in the catalog")?;
        let state = backend.package_state(std::slice::from_ref(&name)).await?;
        if !state
            .packages
            .iter()
            .any(|package| package.installed && package.aoska_manageable)
        {
            bail!("Application is not installed");
        }
        let application = app
            .desktop_application(&name, entry.desktop_id.as_deref())
            .await?
            .context("Application has no registered desktop entry")?;
        tokio::task::spawn_blocking(move || desktop::launch(&name, Some(&application.id)))
            .await??;
        Ok(())
    }
    launch(&app, &backend, catalog_name)
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
