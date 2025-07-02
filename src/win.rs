use crate::cache::{WinsMessage, WinsState};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn get_win(
    Path((key, command)): Path<(String, String)>,
    State(state): State<Arc<WinsState>>,
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

    let cached = match state.cache.get(&key).await {
        Some(cached) => cached.clone(),
        None => 0,
    };

    let value = match args[0].as_str() {
        "_get" => cached,
        "_set" => set(cached, amount),
        "_inc" | "_increment" | "_add" | "_+" => increment(cached, amount),
        "_dec" | "_decrement" | "_-" => decrement(cached, amount),
        _ => cached,
    };

    // New value
    if value != cached {
        state.cache.insert(key, value.clone()).await;

        let msg = WinsMessage { wins: value };
        if let Ok(json) = serde_json::to_string(&msg) {
            let _ = state.sender.send(json);
        }
    }

    Ok(value.to_string())
}

fn set(stored: u32, value: Option<u32>) -> u32 {
    match value {
        Some(value) => value,
        None => stored,
    }
}

fn increment(stored: u32, increment: Option<u32>) -> u32 {
    match increment {
        Some(increment) => stored + increment,
        None => stored + 1,
    }
}

fn decrement(stored: u32, decrement: Option<u32>) -> u32 {
    // Prevent subtract overflow
    if stored == 0 {
        return 0;
    }

    match decrement {
        Some(decrement) => stored - decrement,
        None => stored - 1,
    }
}
