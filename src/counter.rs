use crate::cache::{CounterMessage, CounterState};
use axum::{
    extract::{
        Path, State,
        ws::{CloseFrame, Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

pub async fn command_handler(
    Path((key, command)): Path<(String, String)>,
    State(state): State<Arc<CounterState>>,
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
        None => {
            // Create cache key
            let initial_value = 0;
            state.cache.insert(key.clone(), initial_value).await;
            initial_value
        }
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
        state.cache.insert(key.clone(), value.clone()).await;

        let msg = CounterMessage { counter: value };
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

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(key): Path<String>,
    State(state): State<Arc<CounterState>>,
) -> Response {
    ws.on_upgrade(|socket| ws_handle_socket(socket, key, state))
}

pub async fn ws_handle_socket(mut socket: WebSocket, key: String, state: Arc<CounterState>) {
    let mut rx = state.sender.subscribe();

    // Check if the key exists in the cache
    let cached = match state.cache.get(&key).await {
        Some(cached) => cached,
        None => {
            return {
                let reason = CloseFrame {
                    code: 1008, // Policy Violation
                    reason: Utf8Bytes::from("Invalid key!"),
                };

                let _ = socket.send(Message::Close(Some(reason))).await;
                tracing::info!("invalid websocket: {}", key);
            };
        }
    };

    // Send the current state on connect
    let msg = CounterMessage { counter: cached };
    if let Ok(json) = serde_json::to_string(&msg) {
        let _ = state.sender.send(json);
    }

    while let Ok(msg) = rx.recv().await {
        tracing::debug!("websocket received: {:?}", msg);

        if socket.send(Message::Text(msg.into())).await.is_err() {
            tracing::debug!("websocket closed: {}", key);
            break;
        }
    }
}
