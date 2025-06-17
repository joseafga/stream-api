use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use steam_rs::{Steam, steam_id::SteamId};

pub fn check_your_mom(steamid: u64) -> Option<()> {
    match steamid {
        76561199118689987 => Some(()),
        _ => None,
    }
}

pub async fn get_hours_played(
    Path((steamid, appid)): Path<(u64, u32)>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("getting Steam hours steamid={} appid={}", steamid, appid);
    // Get the Steam API Key as an environment variable.
    let steam_api_key = &std::env::var("STEAM_API_KEY").expect("Missing an API key");
    check_your_mom(steamid).ok_or(StatusCode::BAD_REQUEST)?;

    // Initialize the Steam API client.
    let steam = Steam::new(steam_api_key);
    let steam_id = SteamId::new(steamid);
    let owned_games = steam
        .get_owned_games(steam_id, false, false, 0, false, None, "portuguese", false)
        .await
        .unwrap();

    let game = owned_games
        .games
        .iter()
        .find(|r| r.appid == appid)
        .ok_or(StatusCode::NOT_FOUND)?;

    // println!("{:#?}", game);

    // Convert minutes to hours
    let hours = game.playtime_forever / 60;
    Ok(hours.to_string())
}
