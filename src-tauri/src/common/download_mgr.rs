use reqwest::{
    header::{ACCEPT_RANGES, CONTENT_LENGTH},
    Client,
};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub struct DownloadManager {
    client: Client,
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to download: {0}")]
    Download(String),

    #[error("other error: {0}")]
    Other(#[from] anyhow::Error),
}

impl DownloadManager {
    pub fn new() -> Result<Self, DownloadError> {
        Ok(Self {
            client: Client::builder().user_agent("aoska/1.0").build()?,
        })
    }
    pub fn enqueue(
        &self,
        url: String,
        file_name: String,
        save_path: Option<PathBuf>,
    ) -> tokio::task::JoinHandle<Result<(), DownloadError>> {
        let client = self.client.clone();
        tokio::spawn(async move { Self::download(client, url, file_name, save_path).await })
        // we don't do error handling here.
    }

    async fn download(
        client: Client,
        url: String,
        file_name: String, // NOTE: I don't want to write parser to decode from CONTENT_DISPOSITION. Too complex.
        save_path: Option<PathBuf>,
    ) -> Result<(), DownloadError> {
        let head = client.head(&url).send().await?;
        let dst = match save_path {
            Some(p) => p.clone(),
            None => std::env::temp_dir(),
        };

        let len = head
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        // if the server allows download in range, use multithread downloading.
        let accept_ranges = head
            .headers()
            .get(ACCEPT_RANGES)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.eq_ignore_ascii_case("bytes"))
            .unwrap_or(false);

        if let (Some(size), true) = (len, accept_ranges) {
            // TODO: multithread downloading.
        } else {
            Self::download_single_thread(&client, &url, &dst, &file_name).await?;
        }

        Ok(())
    }

    async fn download_single_thread(
        client: &Client,
        url: &str,
        dst: &Path,
        file_name: &str,
    ) -> Result<(), DownloadError> {
        let mut resp = client.get(url).send().await?.error_for_status()?;
        if !resp.status().is_success() {
            return Err(DownloadError::Download(url.to_string()));
        }
        let mut dst_file = File::create(dst.join(file_name))?;

        // streamly write to disk.
        while let Some(chunk) = resp.chunk().await? {
            dst_file.write_all(&chunk)?;
        }
        Ok(())
    }
}
