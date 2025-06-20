use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use reqwest_tracing::TracingMiddleware;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::instrument;

#[derive(Debug, Serialize)]
struct OwnedGamesRequest {
    steamid: u64,
    include_appinfo: bool,
    include_played_free_games: bool,
    appids_filter: Vec<u32>,
    include_free_sub: Option<bool>,
    language: Option<String>,
    include_extended_appinfo: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OwnedGamesResponse {
    response: OwnedGames,
}

#[derive(Debug, Serialize, Deserialize)]
struct OwnedGames {
    game_count: u32,
    games: Option<Vec<Game>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    appid: u32,
    playtime_forever: u32,
}

#[instrument]
pub async fn get_hours_played(
    Path((steamid, appid)): Path<(u64, u32)>,
) -> Result<impl IntoResponse, StatusCode> {
    check_your_mom(steamid).ok_or(StatusCode::BAD_REQUEST)?;

    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(500), Duration::from_millis(3000))
        .build_with_max_retries(5);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Trace HTTP requests. See the tracing crate to make use of these traces.
        .with(TracingMiddleware::default())
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let owned_games = match get_owned_games(client, steamid, appid).await {
        Ok(owned_games) => owned_games,
        Err(e) => {
            tracing::error!("getting owned games error={:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    tracing::debug!("steam owned_games={:?}", owned_games);

    // Check if the user owns the game
    // Note: will be only one game in the list because we are using `appids_filter`
    let game = owned_games
        .games
        .as_ref()
        .and_then(|g| g.first())
        .ok_or(StatusCode::NOT_FOUND)?;

    if game.appid != appid {
        tracing::error!("Game ID mismatch: expected={}, got={}", appid, game.appid);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Convert minutes to hours
    let hours = game.playtime_forever / 60;
    Ok(hours.to_string())
}

async fn get_owned_games(
    client: ClientWithMiddleware,
    steamid: u64,
    appid: u32,
) -> Result<OwnedGames, Box<dyn std::error::Error>> {
    // Get the Steam API Key as an environment variable.
    let steam_api_key = std::env::var("STEAM_API_KEY").expect("Missing an API key");
    let request = OwnedGamesRequest {
        steamid: steamid,
        include_appinfo: false,
        include_played_free_games: false,
        appids_filter: vec![appid],
        include_free_sub: None,
        language: None, //"pt-BR".to_string(),
        include_extended_appinfo: None,
    };

    // Serialize request params to a JSON string
    let json: String = serde_json::to_string(&request)?;
    let params = [
        ("key", steam_api_key),
        ("format", String::from("json")),
        ("input_json", json),
    ];

    let response = client
        .get("https://api.steampowered.com/IPlayerService/GetOwnedGames/v1")
        .query(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status {}", response.status()).into());
    }

    let owned_games: OwnedGamesResponse = response.json().await?;

    Ok(owned_games.response)
}

fn check_your_mom(steamid: u64) -> Option<()> {
    match steamid {
        76561199118689987 => Some(()),
        _ => None,
    }
}
