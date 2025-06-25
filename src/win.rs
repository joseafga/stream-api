use crate::state::WinsState;
use axum::{Extension, extract::Path, response::IntoResponse};
use reqwest::StatusCode;

pub async fn get_win(
    Path((key, command)): Path<(String, String)>,
    Extension(state): Extension<WinsState>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement restriction
    // check_your_mom(key.as_str()).ok_or(StatusCode::BAD_REQUEST)?;

    // Get vector from Streamelements argument
    let args: Vec<String> = command
        .trim()
        .to_lowercase()
        .split_whitespace()
        .take(2)
        .map(str::to_string)
        .collect();

    // Filter possible second argument
    let amount: Option<u32> = args.get(1).and_then(|s| s.parse::<u32>().ok());
    let mut cache = state.cache.lock().await;

    let value = match cache.get(&key) {
        Some(cached) => cached.clone(),
        None => 0,
    };

    let value = match args[0].as_str() {
        "_get" => value,
        "_set" => win_set(value, amount),
        "_inc" | "_increment" | "_add" | "_+" => win_increment(value, amount),
        "_dec" | "_decrement" | "_-" => win_decrement(value, amount),
        _ => value,
    };

    cache.insert(key, value.clone());

    Ok(value.to_string())
}

fn win_set(cached: u32, value: Option<u32>) -> u32 {
    match value {
        Some(value) => value,
        None => cached,
    }
}

fn win_increment(cached: u32, increment: Option<u32>) -> u32 {
    match increment {
        Some(increment) => cached + increment,
        None => cached + 1,
    }
}

fn win_decrement(cached: u32, decrement: Option<u32>) -> u32 {
    // Prevent subtract overflow
    if cached == 0 {
        return 0;
    }

    match decrement {
        Some(decrement) => cached - decrement,
        None => cached - 1,
    }
}
