use anyhow::Result;
use std::path::Path;
use tauri_plugin_http::reqwest;
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all

pub async fn download_file(url: &str, dest_path: &Path) -> Result<()> {
    let response = reqwest::get(url).await?.error_for_status()?;

    let bytes = response.bytes().await?;

    let mut file = File::create(dest_path).await?;

    file.write_all(&bytes).await?;

    Ok(())
}
