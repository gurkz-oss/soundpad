use anyhow::Result;
use futures_util::StreamExt; // for .next()
use std::path::Path;
use tauri_plugin_http::reqwest;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_file(url: &str, dest_path: &Path) -> Result<()> {
    let response = reqwest::get(url).await?.error_for_status()?;

    let mut file = File::create(dest_path).await?;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let data = chunk?;
        file.write_all(&data).await?;
    }

    Ok(())
}
