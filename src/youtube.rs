use axum::{extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{io::BufRead, process::Command};
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize)]
struct Thumbnail {
    height: Option<u64>,
    url: Option<String>,
    width: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    epoch: u64,
    extractor: String,
    extractor_key: String,
    id: String,
    ie_key: String,
    n_entries: u64,
    original_url: String,
    playlist: String,
    playlist_autonumber: u64,
    playlist_channel: String,
    playlist_channel_id: String,
    playlist_count: u64,
    playlist_id: String,
    playlist_index: u64,
    playlist_title: String,
    playlist_uploader: String,
    playlist_uploader_id: String,
    playlist_webpage_url: String,
    release_year: Value,
    thumbnails: Vec<Thumbnail>,
    title: String,
    url: String,
    view_count: u64,
    webpage_url: String,
    webpage_url_basename: String,
    webpage_url_domain: String,
}

#[instrument]
pub async fn get_last_video(Path(channel): Path<String>) -> Result<String, StatusCode> {
    tracing::debug!("getting last video from channel={}", channel);
    check_your_mom(&channel).ok_or(StatusCode::BAD_REQUEST)?;

    let url = format!("https://www.youtube.com/@{channel}/videos");
    let response = fetch_last_entry(&url).await;

    match response {
        Ok(entry) => Ok(format!("{} - {}", entry.title, entry.url)),
        Err(e) => {
            tracing::error!("Fetch error: {:#?}", e.to_string());
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[instrument]
pub async fn get_last_short(Path(channel): Path<String>) -> Result<String, StatusCode> {
    tracing::debug!("getting last short from channel={}", channel);
    check_your_mom(&channel).ok_or(StatusCode::BAD_REQUEST)?;

    let url = format!("https://www.youtube.com/@{channel}/shorts");
    let response = fetch_last_entry(&url).await;

    match response {
        Ok(entry) => Ok(format!("{} - {}", entry.title, entry.url)),
        Err(e) => {
            tracing::error!("Fetch error: {:#?}", e.to_string());
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn fetch_last_entry(url: &str) -> Result<Entry, Box<dyn std::error::Error>> {
    tracing::debug!("running yt-dlp");

    let output = Command::new("yt-dlp")
        .args(["--dump-json", "--no-download", "--flat-playlist", url])
        .output()
        .expect("Fail on yt-dlp execution.");

    assert!(output.status.success());

    // Yt-dlp return multiples lines with a json each
    let mut lines = output.stdout.lines();
    let line = lines.next().ok_or("Empty output from yt-dlp.")?;
    let json = serde_json::from_str(line.unwrap().as_str())?;

    Ok(json)
}

fn check_your_mom(channel: &String) -> Option<()> {
    match channel.as_str() {
        "raixssa" => Some(()),
        "elakstriker" => Some(()),
        _ => None,
    }
}
