use reqwest::{
    header::{ACCEPT_RANGES, CONTENT_LENGTH, RANGE},
    Client,
};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

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

    #[error("task join error: {0}")]
    ThreadError(#[from] tokio::task::JoinError),

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
        threads: Option<usize>,
    ) -> tokio::task::JoinHandle<Result<(), DownloadError>> {
        let client = self.client.clone();
        tokio::spawn(
            async move { Self::download(client, url, file_name, save_path, threads).await },
        )
        // we don't do error handling here.
    }

    async fn download(
        client: Client,
        url: String,
        file_name: String, // NOTE: I don't want to write parser to decode from CONTENT_DISPOSITION. Too complex.
        save_path: Option<PathBuf>,
        threads: Option<usize>,
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
            Self::download_multi_thread(&client, &url, &dst, &file_name, size, threads).await?;
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
        let mut dst_file = tokio::fs::File::create(dst.join(file_name)).await?;

        // streamly write to disk.
        while let Some(chunk) = resp.chunk().await? {
            dst_file.write_all(&chunk).await?;
        }
        Ok(())
    }

    async fn download_multi_thread(
        client: &Client,
        url: &str,
        dst: &Path,
        file_name: &str,
        size: u64,
        threads: Option<usize>,
    ) -> Result<(), DownloadError> {
        // Maybe check CPU core here to limit threads.
        let threads = threads.unwrap_or(4).max(1);
        let dst = dst.join(file_name);

        // create a temp file.
        let temp_file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&dst)
            .await?;
        temp_file.set_len(size).await?;
        drop(temp_file);

        let chunk_size = size.div_ceil(threads as u64);
        let mut handles: Vec<tokio::task::JoinHandle<Result<(), DownloadError>>> =
            Vec::with_capacity(threads);

        // do parallel download
        let dst = dst.to_path_buf();
        for i in 0..threads {
            let start = i as u64 * chunk_size;
            if start >= size {
                break;
            }
            let end = (start + chunk_size).min(size) - 1;

            let client = client.clone();
            let url = url.to_string();
            let dst_clone = dst.clone();

            handles.push(tokio::spawn(async move {
                let range = format!("bytes={start}-{end}");
                let mut resp = client
                    .get(&url)
                    .header(RANGE, range)
                    .send()
                    .await?
                    .error_for_status()?;
                let mut f = tokio::fs::OpenOptions::new()
                    .write(true)
                    .open(&dst_clone)
                    .await?;
                f.seek(tokio::io::SeekFrom::Start(start)).await?;

                while let Some(chunk) = resp.chunk().await? {
                    f.write_all(&chunk).await?;
                }

                Ok(())
            }));
        }

        for handle in handles {
            handle.await??;
        }
        Ok(())
    }
}
