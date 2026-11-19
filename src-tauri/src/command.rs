use crate::common::{
    amo::AmoBackend,
    amo_types::{
        PmCapabilities, PmOperationLogs, PmOperationStart, PmOperationStatus, PmPackageList,
        PmPlan, PmUpdateSummary, TumUpdateInfo,
    },
    catalog::{installed_entries, repository_entries, CatalogEntry, CatalogIndex},
    config::catalog_endpoint,
    desktop,
    utils::fetch_data,
};
use anyhow::{bail, Context, Result};
use serde_json::Value;

pub struct AppState {
    client: reqwest::Client,
    base_url: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: catalog_endpoint(),
        }
    }
}

#[derive(serde::Deserialize)]
struct RepositoryDetail {
    schema_version: u32,
    id: String,
    installers: Vec<Installer>,
}

#[derive(serde::Deserialize)]
struct Installer {
    kind: String,
    package: Option<RepositoryPackage>,
}

#[derive(serde::Deserialize)]
struct RepositoryPackage {
    names: Vec<String>,
}

impl AppState {
    async fn catalog_entries(&self) -> Result<Vec<CatalogEntry>> {
        let mut index: CatalogIndex =
            fetch_data(&self.client, &self.base_url, "index.json").await?;
        if index.schema_version != 1 {
            bail!("Unsupported ASMR index schema {}", index.schema_version);
        }
        for id in index.mixed_ids() {
            let detail: RepositoryDetail =
                fetch_data(&self.client, &self.base_url, &format!("apps/{id}.json")).await?;
            if detail.id != id || detail.schema_version != 1 {
                bail!("Invalid ASMR detail for {id}");
            }
            let names = detail
                .installers
                .into_iter()
                .filter(|installer| installer.kind == "aosc-repository")
                .filter_map(|installer| installer.package)
                .flat_map(|package| package.names)
                .collect();
            index.set_repository_refs(&id, names);
        }
        Ok(repository_entries(index))
    }
}

#[tauri::command]
pub async fn fetch_catalog_json(
    app: tauri::State<'_, AppState>,
    path: String,
) -> Result<Value, String> {
    if !path.ends_with(".json")
        || !path.split('/').all(|part| {
            !part.is_empty()
                && part != "."
                && part != ".."
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"+._-".contains(&b))
        })
    {
        return Err("Expected a relative ASMR JSON path".into());
    }
    fetch_data(&app.client, &app.base_url, &path)
        .await
        .map_err(|e| format!("{e:#}"))
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
    if result.packages.iter().any(|pkg| pkg.installed) {
        if let Ok(entries) = app.catalog_entries().await {
            for package in result.packages.iter_mut().filter(|pkg| pkg.installed) {
                let entry = entries
                    .iter()
                    .find(|app| app.package_refs.contains(&package.name));
                if let Some(entry) = entry {
                    match desktop::resolve(&package.name, entry.desktop_id.as_deref()) {
                        Ok(application) => package.openable = application.is_some(),
                        Err(error) => eprintln!("Desktop lookup for {}: {error:#}", package.name),
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
        let names = crate::common::catalog::package_names(&entries);
        let states = backend.package_state(&names).await?;
        let mut applications = installed_entries(entries, states.packages, &app.base_url)?;
        for application in &mut applications {
            match desktop::resolve(&application.package.name, application.desktop_id.as_deref()) {
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
    package_name: Option<String>,
) -> Result<(), String> {
    async fn launch(
        app: &AppState,
        backend: &AmoBackend,
        id: String,
        selected: Option<String>,
    ) -> Result<()> {
        let entry = app
            .catalog_entries()
            .await?
            .into_iter()
            .find(|app| app.id == id)
            .context("Application is not in the repository catalog")?;
        let names = entry.launch_candidates(selected.as_deref())?;
        let states = backend.package_state(&names).await?;
        let mut installed = states
            .packages
            .into_iter()
            .filter(|pkg| pkg.installed && pkg.aoska_manageable)
            .peekable();
        if installed.peek().is_none() {
            bail!("Application is not installed");
        }
        for package in installed {
            if let Some(application) = desktop::resolve(&package.name, entry.desktop_id.as_deref())?
            {
                tokio::task::spawn_blocking(move || {
                    desktop::launch(&package.name, Some(&application.id))
                })
                .await??;
                return Ok(());
            }
        }
        bail!("Application has no registered desktop entry")
    }
    launch(&app, &backend, catalog_name, package_name)
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
