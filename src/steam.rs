use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

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

pub async fn get_hours_played(
    Path((steamid, appid)): Path<(u64, u32)>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("getting Steam hours steamid={} appid={}", steamid, appid);
    check_your_mom(steamid).ok_or(StatusCode::BAD_REQUEST)?;

    let owned_games = match get_owned_games(steamid, appid).await {
        Ok(owned_games) => owned_games,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Check if the user owns the game
    // Note: will be only one game in the list because we are using `appids_filter`
    let game = owned_games
        .games
        .as_ref()
        .and_then(|g| g.first())
        .ok_or(StatusCode::NOT_FOUND)?;

    assert_eq!(game.appid, appid);

    // Convert minutes to hours
    let hours = game.playtime_forever / 60;
    Ok(hours.to_string())
}

async fn get_owned_games(
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

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.steampowered.com/IPlayerService/GetOwnedGames/v1")
        .query(&params)
        .send()
        .await?;

    let owned_games: OwnedGamesResponse = response.json().await?;
    tracing::debug!("steam owned_games={:?}", owned_games.response);

    Ok(owned_games.response)
}

fn check_your_mom(steamid: u64) -> Option<()> {
    match steamid {
        76561199118689987 => Some(()),
        _ => None,
    }
}
