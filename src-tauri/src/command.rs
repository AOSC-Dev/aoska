use crate::common::omactl;
use crate::common::{
    config::{ASM_ENDPOINT, ASM_INDEX_PATH, ASM_RECOMMEND_INDEX_PATH},
    index::{CategoryIndex, Index, RecommendIndex},
    omactl_types::{PmCapabilities, PmOperationStart, PmUpdateSummary},
    packages::{Category, PackageDetail},
    utils::fetch_data,
};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::{Command as StdCommand, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::Emitter; // windows.emit

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
pub async fn fetch_update_count(_app: tauri::State<'_, AppState>) -> Result<usize, String> {
    pm_update_summary().await.map(|summary| summary.total)
}

#[tauri::command]
pub async fn fetch_update_detail(_app: tauri::State<'_, AppState>) -> Result<Value, String> {
    pm_list_updates().await
}

#[tauri::command]
pub async fn fetch_tum_update(_app: tauri::State<'_, AppState>) -> Result<Vec<Value>, String> {
    // TUM/security classification is intentionally unavailable until omactl exposes
    // plan.tum.v1 data through the JSON contract consumed by pm_* commands.
    Ok(Vec::new())
}

#[tauri::command]
pub async fn get_endpoint_base_url(app: tauri::State<'_, AppState>) -> Result<String, String> {
    Ok(app.base_url.clone())
}

// Report whether oma is currently busy.
#[tauri::command]
pub async fn oma_is_busy() -> Result<bool, String> {
    Ok(omactl::is_busy())
}

fn join_blocking<T>(
    result: Result<Result<T, anyhow::Error>, tokio::task::JoinError>,
) -> Result<T, String> {
    result
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pm_capabilities() -> Result<PmCapabilities, String> {
    join_blocking(tokio::task::spawn_blocking(omactl::capabilities).await)
}

#[tauri::command]
pub async fn pm_list_updates() -> Result<Value, String> {
    join_blocking(tokio::task::spawn_blocking(omactl::query_upgradable).await)
}

#[tauri::command]
pub async fn pm_update_summary() -> Result<PmUpdateSummary, String> {
    join_blocking(tokio::task::spawn_blocking(omactl::update_summary).await)
}

#[tauri::command]
pub async fn pm_list_installed() -> Result<Value, String> {
    join_blocking(tokio::task::spawn_blocking(omactl::query_installed).await)
}

#[tauri::command]
pub async fn pm_package_state(packages: Vec<String>) -> Result<Value, String> {
    join_blocking(
        tokio::task::spawn_blocking(move || omactl::query_package_detail(&packages)).await,
    )
}

#[tauri::command]
pub async fn pm_start_update(
    packages: Option<Vec<String>>,
    assume_yes: Option<bool>,
) -> Result<PmOperationStart, String> {
    let packages = packages.unwrap_or_default();
    join_blocking(
        tokio::task::spawn_blocking(move || {
            omactl::run_upgrade(&packages, assume_yes.unwrap_or(true))
        })
        .await,
    )
}

#[tauri::command]
pub async fn pm_start_install(
    packages: Vec<String>,
    assume_yes: Option<bool>,
) -> Result<PmOperationStart, String> {
    join_blocking(
        tokio::task::spawn_blocking(move || {
            omactl::run_install(&packages, assume_yes.unwrap_or(true))
        })
        .await,
    )
}

#[tauri::command]
pub async fn pm_start_remove(
    packages: Vec<String>,
    remove_config: Option<bool>,
    assume_yes: Option<bool>,
) -> Result<PmOperationStart, String> {
    join_blocking(
        tokio::task::spawn_blocking(move || {
            omactl::run_remove(
                &packages,
                remove_config.unwrap_or(true),
                assume_yes.unwrap_or(true),
            )
        })
        .await,
    )
}

#[tauri::command]
pub async fn pm_start_refresh() -> Result<PmOperationStart, String> {
    join_blocking(tokio::task::spawn_blocking(omactl::run_refresh).await)
}

#[tauri::command]
pub async fn pm_operation_status(unit: String) -> Result<Value, String> {
    join_blocking(tokio::task::spawn_blocking(move || omactl::status(&unit)).await)
}

#[tauri::command]
pub async fn pm_operation_result(unit: String) -> Result<Value, String> {
    join_blocking(tokio::task::spawn_blocking(move || omactl::result(&unit)).await)
}

#[tauri::command]
pub async fn pm_operation_logs(unit: String) -> Result<Value, String> {
    join_blocking(tokio::task::spawn_blocking(move || omactl::logs(&unit)).await)
}

#[tauri::command]
pub async fn pm_cancel_operation(unit: String) -> Result<Value, String> {
    join_blocking(tokio::task::spawn_blocking(move || omactl::cancel(&unit)).await)
}

// Start a system upgrade via omactl, returning the systemd unit name.
#[tauri::command]
pub async fn start_upgrade(
    packages: Option<Vec<String>>,
    wait: Option<bool>,
    follow: Option<bool>,
    unit: Option<String>,
    assume_yes: Option<bool>,
) -> Result<String, String> {
    let mut args: Vec<&str> = vec!["upgrade"];
    if assume_yes.unwrap_or(true) {
        args.push("--yes");
    }
    if let Some(pkgs) = &packages {
        if !pkgs.is_empty() {
            args.extend(pkgs.iter().map(|s| s.as_str()));
        }
    }
    omactl::run_oma(
        &args,
        wait.unwrap_or(false),
        follow.unwrap_or(false),
        unit.as_deref(),
    )
    .map_err(|e| e.to_string())
}

// Start installing packages via omactl, returning the systemd unit name.
// packages must be non-empty.
#[tauri::command]
pub async fn start_install(
    packages: Vec<String>,
    wait: Option<bool>,
    follow: Option<bool>,
    unit: Option<String>,
    assume_yes: Option<bool>,
) -> Result<String, String> {
    if packages.is_empty() {
        return Err("packages is empty".to_string());
    }
    let mut args: Vec<&str> = vec!["install"];
    if assume_yes.unwrap_or(true) {
        args.push("--yes");
    }
    let pkg_refs: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
    args.extend(pkg_refs);
    omactl::run_oma(
        &args,
        wait.unwrap_or(false),
        follow.unwrap_or(false),
        unit.as_deref(),
    )
    .map_err(|e| e.to_string())
}

// Start removing packages via omactl, return the unit name.
#[tauri::command]
pub async fn start_remove(
    packages: Vec<String>,
    // purge, remove app config or not.
    remove_config: Option<bool>,
    wait: Option<bool>,
    follow: Option<bool>,
    unit: Option<String>,
    assume_yes: Option<bool>,
) -> Result<String, String> {
    if packages.is_empty() {
        return Err("packages is empty".to_string());
    }
    let mut args: Vec<&str> = vec!["remove"];
    if assume_yes.unwrap_or(true) {
        args.push("--yes");
    }
    if remove_config.unwrap_or(true) {
        args.push("--remove_config");
    }
    let pkg_refs: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
    args.extend(pkg_refs);
    omactl::run_oma(
        &args,
        wait.unwrap_or(false),
        follow.unwrap_or(false),
        unit.as_deref(),
    )
    .map_err(|e| e.to_string())
}

/// Fetch a unit's current status.
#[tauri::command]
pub async fn oma_unit_status(unit: String) -> Result<String, String> {
    let value = pm_operation_status(unit).await?;
    serde_json::to_string(&value).map_err(|e| e.to_string())
}

/// Fetch a unit's accumulated logs.
#[tauri::command]
pub async fn oma_unit_logs(unit: String) -> Result<String, String> {
    let value = pm_operation_logs(unit).await?;
    serde_json::to_string(&value).map_err(|e| e.to_string())
}

/// Fetch a unit's result.
#[tauri::command]
pub async fn oma_unit_result(unit: String) -> Result<String, String> {
    let value = pm_operation_result(unit).await?;
    serde_json::to_string(&value).map_err(|e| e.to_string())
}

// Store active log-follow cancel senders so we can stop them.
type StopSender = std::sync::mpsc::Sender<()>;
type FollowerT = Arc<Mutex<HashMap<String, StopSender>>>;
static FOLLOWERS: Lazy<FollowerT> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

fn followers_map() -> FollowerT {
    FOLLOWERS.clone()
}

#[derive(serde::Serialize, Clone)]
pub struct FollowerMsg {
    pub unit: String,
    pub line: String,
}

/// Start following a unit's logs and emit them to the frontend in real-time.
/// Event name: "oma-log".
/// Payload JSON: { unit: String, line: String }
/// If already (this wouldn't happen in design.) following the unit, returns Ok immediately.
#[tauri::command]
pub async fn follow_oma_logs(window: tauri::Window, unit: String) -> Result<(), String> {
    let map = followers_map();
    {
        let mut guard = map.lock().unwrap();
        if guard.contains_key(&unit) {
            return Ok(()); // already following, return.
        }
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        guard.insert(unit.clone(), tx); // record.

        let win = window.clone();
        thread::spawn(move || {
            let mut cmd = StdCommand::new("omactl");
            cmd.args(["logs", "--json", "--follow", &unit.clone()])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            if let Ok(mut child) = cmd.spawn() {
                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line_res in reader.lines() {
                        if rx.try_recv().is_ok() {
                            break;
                        }
                        if let Ok(line) = line_res {
                            let log_msg = FollowerMsg {
                                unit: unit.clone(),
                                line,
                            };
                            let _ = win.emit("oma-log", log_msg);
                        } else {
                            break;
                        }
                    }
                }
                let _ = child.kill();
            } else {
                let log_msg = FollowerMsg {
                    unit: unit.clone(),
                    line: "<failed to spawn omactl logs --json --follow>".to_string(),
                };
                let _ = win.emit("oma-log", log_msg);
            }
            // remove the map from Followers
            let map = followers_map();
            let mut guard = map.lock().unwrap();
            guard.remove(&unit.clone());
        });
    }
    Ok(())
}

/// Stop following a unit's logs.
#[tauri::command]
pub async fn stop_follow_oma_logs(unit: String) -> Result<(), String> {
    let map = followers_map();
    let mut guard = map.lock().unwrap();
    if let Some(sender) = guard.remove(&unit) {
        let _ = sender.send(()); // signal stop
    }
    Ok(())
}
