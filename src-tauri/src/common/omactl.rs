use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use thiserror::Error;

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

#[derive(Error, Debug)]
#[error("oma is busy {0}")]
pub struct OmaBusyError(String);

impl OmaBusyError {
    pub fn new(unit: Option<&str>) -> Self {
        let s = unit.map(|u| format!(" (unit={u})")).unwrap_or_default();
        Self(s)
    }
}

pub fn is_busy() -> bool {
    let lock_path = "/run/lock/oma.lock";
    Path::new(lock_path).exists()
}

/// Run an oma task via omactl. Returns the unit name created by omactl.
pub fn run_oma(args: &[&str], wait: bool, follow: bool, unit: Option<&str>) -> Result<String> {
    // TODO: race condition ? there must be a better solution.
    if is_busy() {
        return Err(OmaBusyError::new(unit).into());
    }
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
