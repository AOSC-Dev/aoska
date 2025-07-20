use anyhow::{Context, Ok, Result};
use reqwest::Client;

use serde::de::DeserializeOwned;

fn build_url(endpoint:&str, path: &str) -> String {
    format!(
        "{}/{}",
        endpoint.trim_end_matches("/"),
        path.trim_end_matches("/")
    )
}

pub async fn fetch_data<T>(client: &Client, endpoint:&str, path: &str) -> Result<T>
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

    resp.error_for_status_ref().with_context(|| format!("Bad Status: {}", resp.status()))?;

    // Deserialize Json
    let data = resp.json::<T>().await.with_context(|| format!("Invalid JSON in {path}"))?;
    Ok(data)
}
