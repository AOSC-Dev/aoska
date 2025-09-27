use anyhow::{Context, Ok, Result};
use reqwest::Client;
use std::process::Command;

use serde::de::DeserializeOwned;

fn build_url(endpoint: &str, path: &str) -> String {
    format!(
        "{}/{}",
        endpoint.trim_end_matches("/"),
        path.trim_end_matches("/")
    )
}

pub async fn fetch_data<T>(client: &Client, endpoint: &str, path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let url = build_url(endpoint, path);
    let resp = client
        .get(&url)
        .header("User-Agent", "aoska/1.0")
        .send()
        .await
        .with_context(|| format!("GET {url}"))?;

    resp.error_for_status_ref()
        .with_context(|| format!("Bad Status: {}", resp.status()))?;

    // Deserialize Json
    let data = resp
        .json::<T>()
        .await
        .with_context(|| format!("Invalid JSON in {path}"))?;
    Ok(data)
}

pub fn run_cmd(mut cmd: Command) -> Result<String> {
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
