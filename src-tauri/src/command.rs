use crate::common::omactl;
use crate::common::{
    config::{ASM_ENDPOINT, ASM_INDEX_PATH, ASM_RECOMMEND_INDEX_PATH},
    index::{CategoryIndex, Index, RecommendIndex},
    omactl_types::{PmCapabilities, PmOperationStart, PmUpdateSummary, TumUpdateInfo},
    packages::{Category, PackageDetail},
    utils::fetch_data,
};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
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
    let updates = pm_list_updates().await?;
    Ok(compat_oma_operation(&updates))
}

#[tauri::command]
pub async fn fetch_tum_update(
    _app: tauri::State<'_, AppState>,
) -> Result<Vec<TumUpdateInfo>, String> {
    join_blocking(tokio::task::spawn_blocking(omactl::query_tum_updates).await)
}

fn compat_oma_operation(updates: &Value) -> Value {
    let packages = updates
        .get("packages")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let install: Vec<Value> = packages
        .iter()
        .enumerate()
        .map(|(index, pkg)| {
            let name = pkg
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let arch = pkg
                .get("architecture")
                .or_else(|| pkg.get("arch"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let name_without_arch = pkg
                .get("name_without_arch")
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| name.split(':').next().unwrap_or(&name).to_string());
            json!({
                "name": name,
                "name_without_arch": name_without_arch,
                "old_version": pkg.get("current_version").cloned().unwrap_or(Value::Null),
                "new_version": pkg
                    .get("new_version")
                    .or_else(|| pkg.get("version"))
                    .cloned()
                    .unwrap_or_else(|| Value::String(String::new())),
                "old_size": pkg.get("old_size").cloned().unwrap_or(Value::Null),
                "new_size": pkg.get("new_size").and_then(Value::as_u64).unwrap_or(0),
                "pkg_urls": [],
                "sha256": pkg.get("sha256").cloned().unwrap_or(Value::Null),
                "md5": pkg.get("md5").cloned().unwrap_or(Value::Null),
                "sha512": pkg.get("sha512").cloned().unwrap_or(Value::Null),
                "arch": arch,
                "download_size": pkg.get("download_size").and_then(Value::as_u64).unwrap_or(0),
                "op": 3,
                "automatic": pkg.get("automatic").and_then(Value::as_bool).unwrap_or(false),
                "index": index,
            })
        })
        .collect();
    json!({
        "install": install,
        "remove": [],
        "disk_size_delta": 0,
        "autoremovable": [0, 0],
        "total_download_size": 0,
        "suggest": [],
        "recommend": [],
    })
}

#[tauri::command]
pub async fn get_endpoint_base_url(app: tauri::State<'_, AppState>) -> Result<String, String> {
    Ok(app.base_url.clone())
}

// Deprecated compatibility command. Busy state is exposed by structured omactl
// operation errors such as OMA_BUSY, not by reading oma lock files in aoska.
#[tauri::command]
pub async fn oma_is_busy() -> Result<bool, String> {
    Err("UNSUPPORTED_COMMAND: oma_is_busy is not available through omactl JSON".to_string())
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

async fn require_pm_capability(capability: &'static str) -> Result<(), String> {
    join_blocking(tokio::task::spawn_blocking(move || omactl::require_capability(capability)).await)
}

#[tauri::command]
pub async fn pm_list_updates() -> Result<Value, String> {
    require_pm_capability("query.upgradable.v1").await?;
    join_blocking(tokio::task::spawn_blocking(omactl::query_upgradable).await)
}

#[tauri::command]
pub async fn pm_update_summary() -> Result<PmUpdateSummary, String> {
    require_pm_capability("query.upgradable.v1").await?;
    join_blocking(tokio::task::spawn_blocking(omactl::update_summary).await)
}

#[tauri::command]
pub async fn pm_list_installed() -> Result<Value, String> {
    require_pm_capability("query.installed.v1").await?;
    join_blocking(tokio::task::spawn_blocking(omactl::query_installed).await)
}

#[tauri::command]
pub async fn pm_package_state(packages: Vec<String>) -> Result<Value, String> {
    require_pm_capability("query.package-detail.v1").await?;
    join_blocking(
        tokio::task::spawn_blocking(move || omactl::query_package_detail(&packages)).await,
    )
}

#[tauri::command]
pub async fn pm_plan_update(packages: Option<Vec<String>>) -> Result<Value, String> {
    let packages = packages.unwrap_or_default();
    if packages.is_empty() {
        require_pm_capability("plan.upgrade.v1").await?;
    } else {
        require_pm_capability("plan.upgrade.selected.v1").await?;
    }
    join_blocking(tokio::task::spawn_blocking(move || omactl::plan_upgrade(&packages)).await)
}

#[tauri::command]
pub async fn pm_plan_install(packages: Vec<String>) -> Result<Value, String> {
    require_pm_capability("plan.install.v1").await?;
    join_blocking(tokio::task::spawn_blocking(move || omactl::plan_install(&packages)).await)
}

#[tauri::command]
pub async fn pm_plan_remove(packages: Vec<String>) -> Result<Value, String> {
    require_pm_capability("plan.remove.v1").await?;
    join_blocking(tokio::task::spawn_blocking(move || omactl::plan_remove(&packages)).await)
}

#[tauri::command]
pub async fn pm_start_update(
    packages: Option<Vec<String>>,
    assume_yes: Option<bool>,
    unit: Option<String>,
) -> Result<PmOperationStart, String> {
    let packages = packages.unwrap_or_default();
    if packages.is_empty() {
        require_pm_capability("run.upgrade.v1").await?;
    } else {
        require_pm_capability("run.upgrade.selected.v1").await?;
    }
    join_blocking(
        tokio::task::spawn_blocking(move || {
            omactl::run_upgrade(&packages, assume_yes.unwrap_or(true), unit.as_deref())
        })
        .await,
    )
}

#[tauri::command]
pub async fn pm_start_install(
    packages: Vec<String>,
    assume_yes: Option<bool>,
    unit: Option<String>,
) -> Result<PmOperationStart, String> {
    require_pm_capability("run.install.v1").await?;
    join_blocking(
        tokio::task::spawn_blocking(move || {
            omactl::run_install(&packages, assume_yes.unwrap_or(true), unit.as_deref())
        })
        .await,
    )
}

#[tauri::command]
pub async fn pm_start_remove(
    packages: Vec<String>,
    remove_config: Option<bool>,
    assume_yes: Option<bool>,
    unit: Option<String>,
) -> Result<PmOperationStart, String> {
    require_pm_capability("run.remove.v1").await?;
    join_blocking(
        tokio::task::spawn_blocking(move || {
            omactl::run_remove(
                &packages,
                remove_config.unwrap_or(true),
                assume_yes.unwrap_or(true),
                unit.as_deref(),
            )
        })
        .await,
    )
}

#[tauri::command]
pub async fn pm_start_refresh(
    purpose: Option<String>,
    unit: Option<String>,
) -> Result<PmOperationStart, String> {
    require_pm_capability("run.refresh.v1").await?;
    let purpose = purpose.unwrap_or_else(|| "check-updates".to_string());
    join_blocking(
        tokio::task::spawn_blocking(move || omactl::run_refresh(&purpose, unit.as_deref())).await,
    )
}

#[tauri::command]
pub async fn pm_operation_status(unit: String) -> Result<Value, String> {
    require_pm_capability("unit.status.v1").await?;
    join_blocking(tokio::task::spawn_blocking(move || omactl::status(&unit)).await)
}

#[tauri::command]
pub async fn pm_operation_result(unit: String) -> Result<Value, String> {
    require_pm_capability("unit.result.v1").await?;
    join_blocking(tokio::task::spawn_blocking(move || omactl::result(&unit)).await)
}

#[tauri::command]
pub async fn pm_operation_logs(unit: String) -> Result<Value, String> {
    require_pm_capability("unit.logs.v1").await?;
    join_blocking(tokio::task::spawn_blocking(move || omactl::logs(&unit)).await)
}

#[tauri::command]
pub async fn pm_cancel_operation(unit: String) -> Result<Value, String> {
    require_pm_capability("unit.cancel.v1").await?;
    join_blocking(tokio::task::spawn_blocking(move || omactl::cancel(&unit)).await)
}

#[tauri::command]
pub async fn pm_follow_operation_logs(window: tauri::Window, unit: String) -> Result<(), String> {
    require_pm_capability("unit.logs.v1").await?;
    start_follow_operation_logs(window, unit, false)
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
    if wait.unwrap_or(false) || follow.unwrap_or(false) {
        return Err(
            "wait/follow are not supported by the omactl JSON compatibility wrapper".into(),
        );
    }
    pm_start_update(packages, assume_yes, unit)
        .await
        .map(|started| started.unit)
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
    if wait.unwrap_or(false) || follow.unwrap_or(false) {
        return Err(
            "wait/follow are not supported by the omactl JSON compatibility wrapper".into(),
        );
    }
    pm_start_install(packages, assume_yes, unit)
        .await
        .map(|started| started.unit)
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
    if wait.unwrap_or(false) || follow.unwrap_or(false) {
        return Err(
            "wait/follow are not supported by the omactl JSON compatibility wrapper".into(),
        );
    }
    pm_start_remove(packages, remove_config, assume_yes, unit)
        .await
        .map(|started| started.unit)
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

#[derive(serde::Serialize, Clone)]
pub struct FollowerErrorMsg {
    pub unit: String,
    pub message: String,
}

fn follower_process_error(
    unit: &str,
    success: bool,
    code: Option<i32>,
    stderr: &str,
    saw_stdout: bool,
) -> Option<String> {
    if success {
        return None;
    }

    let output_state = if saw_stdout {
        "after log output"
    } else {
        "without log output"
    };
    let status = code
        .map(|code| format!("status={code}"))
        .unwrap_or_else(|| "status=terminated".to_string());
    let stderr = stderr.trim();
    let stderr = if stderr.is_empty() {
        String::new()
    } else {
        format!(" stderr={stderr}")
    };

    Some(format!(
        "omactl logs --json --follow failed for {unit} {output_state}: {status}{stderr}"
    ))
}

fn emit_follower_error(
    win: &tauri::Window,
    unit: &str,
    message: String,
    emit_legacy_oma_log: bool,
) {
    let _ = win.emit(
        "pm-operation-log-error",
        FollowerErrorMsg {
            unit: unit.to_string(),
            message: message.clone(),
        },
    );
    if emit_legacy_oma_log {
        let _ = win.emit(
            "oma-log",
            FollowerMsg {
                unit: unit.to_string(),
                line: format!("<{message}>"),
            },
        );
    }
}

fn start_follow_operation_logs(
    window: tauri::Window,
    unit: String,
    emit_legacy_oma_log: bool,
) -> Result<(), String> {
    let map = followers_map();
    let mut guard = map.lock().unwrap();
    if guard.contains_key(&unit) {
        return Ok(());
    }
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    guard.insert(unit.clone(), tx);
    drop(guard);

    let win = window.clone();
    thread::spawn(move || {
        let mut cmd = StdCommand::new("omactl");
        cmd.args(["logs", "--json", "--follow", &unit])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Ok(mut child) = cmd.spawn() {
            let stderr_handle = child.stderr.take().map(|stderr| {
                thread::spawn(move || {
                    let mut reader = BufReader::new(stderr);
                    let mut stderr = String::new();
                    let _ = reader.read_to_string(&mut stderr);
                    stderr
                })
            });
            let mut saw_stdout = false;
            let mut should_kill = false;
            let mut emitted_error = false;
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line_res in reader.lines() {
                    if rx.try_recv().is_ok() {
                        should_kill = true;
                        break;
                    }
                    match line_res {
                        Ok(line) => match omactl::validate_log_event(&line, &unit) {
                            Ok(event) => {
                                saw_stdout = true;
                                let _ = win.emit("pm-operation-log", event.clone());
                                if emit_legacy_oma_log {
                                    let event_line = event
                                        .get("data")
                                        .and_then(|data| data.get("line"))
                                        .and_then(Value::as_str)
                                        .unwrap_or(&line)
                                        .to_string();
                                    let _ = win.emit(
                                        "oma-log",
                                        FollowerMsg {
                                            unit: unit.clone(),
                                            line: event_line,
                                        },
                                    );
                                }
                            }
                            Err(error) => {
                                emit_follower_error(
                                    &win,
                                    &unit,
                                    error.to_string(),
                                    emit_legacy_oma_log,
                                );
                                emitted_error = true;
                                should_kill = true;
                                break;
                            }
                        },
                        Err(error) => {
                            emit_follower_error(
                                &win,
                                &unit,
                                error.to_string(),
                                emit_legacy_oma_log,
                            );
                            emitted_error = true;
                            should_kill = true;
                            break;
                        }
                    }
                }
            }
            if rx.try_recv().is_ok() {
                should_kill = true;
            }
            if should_kill {
                let _ = child.kill();
            }
            let wait_result = child.wait();
            let stderr = stderr_handle
                .and_then(|handle| handle.join().ok())
                .unwrap_or_default();
            match wait_result {
                Ok(status) if !emitted_error => {
                    if let Some(message) = follower_process_error(
                        &unit,
                        status.success(),
                        status.code(),
                        &stderr,
                        saw_stdout,
                    ) {
                        emit_follower_error(&win, &unit, message, emit_legacy_oma_log);
                    }
                }
                Err(error) if !emitted_error => {
                    emit_follower_error(
                        &win,
                        &unit,
                        format!("failed to reap omactl logs --json --follow: {error}"),
                        emit_legacy_oma_log,
                    );
                }
                _ => {}
            }
        } else {
            let message = "failed to spawn omactl logs --json --follow".to_string();
            emit_follower_error(&win, &unit, message, emit_legacy_oma_log);
        }
        let map = followers_map();
        let mut guard = map.lock().unwrap();
        guard.remove(&unit);
    });
    Ok(())
}

/// Start following a unit's logs and emit them to the frontend in real-time.
/// Event name: "oma-log".
/// Payload JSON: { unit: String, line: String }
/// If already (this wouldn't happen in design.) following the unit, returns Ok immediately.
#[tauri::command]
pub async fn follow_oma_logs(window: tauri::Window, unit: String) -> Result<(), String> {
    require_pm_capability("unit.logs.v1").await?;
    start_follow_operation_logs(window, unit, true)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follower_process_error_reports_nonzero_stderr_without_stdout() {
        let message = follower_process_error(
            "oma-task-test.service",
            false,
            Some(42),
            "omactl exploded\n",
            false,
        )
        .unwrap();

        assert!(message.contains("oma-task-test.service"));
        assert!(message.contains("status=42"));
        assert!(message.contains("omactl exploded"));
        assert!(message.contains("without log output"));
    }

    #[test]
    fn follower_process_error_ignores_success() {
        assert!(
            follower_process_error("oma-task-test.service", true, Some(0), "", false).is_none()
        );
    }
}
