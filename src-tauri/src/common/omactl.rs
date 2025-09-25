use anyhow::{Context, Result};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    LazyLock,
};
use tokio::task;

fn run_cmd(mut cmd: Command) -> Result<String> {
    let out = cmd.output().context("failed to spawn command")?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let msg = format!(
            "command failed: status={:?}\nstdout:\n{}\nstderr:\n{}",
            out.status.code(),
            stdout,
            stderr
        );
        anyhow::bail!(msg)
    }
}

// Global busy state and mutex
static OMA_BUSY: AtomicBool = AtomicBool::new(false);
pub static OMA_MUTEX: LazyLock<tokio::sync::Mutex<()>> =
    LazyLock::new(|| tokio::sync::Mutex::new(()));

pub fn is_busy() -> bool {
    OMA_BUSY.load(Ordering::SeqCst)
}

/// Run an oma task via omactl. Returns the unit name created by omactl.
pub fn run_oma(args: &[&str], wait: bool, follow: bool, unit: Option<&str>) -> Result<String> {
    let mut cmd = Command::new("omactl");
    cmd.arg("run");
    if wait {
        cmd.arg("--wait");
    }
    if follow {
        cmd.arg("--follow");
    }
    if let Some(u) = unit {
        cmd.arg(format!("--unit={u}"));
    }
    cmd.arg("--");
    cmd.arg("oma");
    cmd.args(args);

    let out = run_cmd(cmd)?;
    // unit=oma-task-YYYYmmddHHMMSS-<rand>
    let unit = out
        .lines()
        .find_map(|l| l.strip_prefix("unit="))
        .unwrap_or("")
        .trim()
        .to_string();
    Ok(if unit.is_empty() { out } else { unit })
}

/// Run an oma task via omactl. Returns the unit name created by omactl.
/// This function always uses wait=true to keep the lock until oma finishes.
pub async fn run_oma_blocking(args: &[&str], follow: bool, unit: Option<&str>) -> Result<String> {
    let _guard = OMA_MUTEX.lock().await;
    OMA_BUSY.store(true, Ordering::SeqCst);
    // Run in a blocking thread to avoid another oma instance running in the same time.
    let args_owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let unit_owned = unit.map(|s| s.to_string());
    let res = task::spawn_blocking(move || {
        let args_ref: Vec<&str> = args_owned.iter().map(|s| s.as_str()).collect();
        run_oma(&args_ref, true, follow, unit_owned.as_deref())
    })
    .await
    .unwrap_or_else(|e| Err(anyhow::anyhow!(e.to_string())));
    OMA_BUSY.store(false, Ordering::SeqCst);
    res
}

pub fn list_units() -> Result<String> {
    run_cmd({
        let mut c = Command::new("omactl");
        c.arg("list");
        c
    })
}

pub fn status(unit: &str) -> Result<String> {
    run_cmd({
        let mut c = Command::new("omactl");
        c.arg("status").arg(unit);
        c
    })
}

pub fn logs(unit: &str) -> Result<String> {
    run_cmd({
        let mut c = Command::new("omactl");
        c.arg("logs").arg(unit);
        c
    })
}

pub fn result(unit: &str) -> Result<String> {
    run_cmd({
        let mut c = Command::new("omactl");
        c.arg("result").arg(unit);
        c
    })
}

pub fn cancel(unit: &str) -> Result<String> {
    run_cmd({
        let mut c = Command::new("omactl");
        c.arg("cancel").arg(unit);
        c
    })
}
