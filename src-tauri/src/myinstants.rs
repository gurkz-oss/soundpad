use serde::Deserialize;
use tauri_plugin_http::reqwest;

#[derive(Debug, Deserialize)]
pub struct MyInstantsResponse {
    pub status: String,
    pub author: String,
    pub data: InstantData,
}

#[derive(Debug, Deserialize)]
pub struct InstantData {
    pub id: String,
    pub url: String,
    pub title: String,
    pub mp3: String,
    pub description: String,
    pub tags: Vec<String>,
    pub favorites: String,
    pub views: String,
    pub uploader: Uploader,
}

#[derive(Debug, Deserialize)]
pub struct Uploader {
    pub text: Option<String>, // <-- make optional to handle null
    pub url: String,
}

/// Finds the first balanced {...} block and returns it as &str.
/// this strips any stray HTML/PHP warnings, which the api returns sometimes because its dumb
fn extract_json(input: &str) -> Option<&str> {
    let mut depth = 0usize;
    let mut start = None;

    for (i, c) in input.char_indices() {
        match c {
            '{' => {
                if start.is_none() {
                    start = Some(i);
                }
                depth += 1;
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                    if depth == 0 {
                        let s = start?;
                        return Some(&input[s..=i]);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Convert a MyInstants web URL to the API endpoint.
pub fn myinstants_to_api_url(url: &str) -> Option<String> {
    let trimmed = url.trim_end_matches('/');
    let last_segment = trimmed.rsplit('/').next()?;
    Some(format!(
        "https://myinstants-api.vercel.app/detail?id={}",
        last_segment
    ))
}

/// Fetch and return the `InstantData` for a MyInstants page URL.
pub async fn get_data(url: &str) -> Result<InstantData, Box<dyn std::error::Error>> {
    // Build API endpoint
    let api_url = myinstants_to_api_url(url).ok_or("Invalid input URL")?;

    // Fetch raw response
    let raw = reqwest::get(&api_url).await?.text().await?;

    // Extract valid JSON and deserialize
    let json_str = extract_json(&raw).ok_or("No valid JSON found")?;
    let parsed: MyInstantsResponse = serde_json::from_str(json_str)?;

    Ok(parsed.data)
}
