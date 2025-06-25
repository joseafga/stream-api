use crate::steam::OwnedGames;
use axum::{Extension, Router, error_handling::HandleErrorLayer, http::StatusCode, routing::get};
use axum_response_cache::CacheLayer;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

mod sentence;
mod steam;
mod win;
mod youtube;

// Cache shared across requests
#[derive(Debug, Clone)]
struct State<T> {
    cache: Arc<Mutex<HashMap<String, T>>>,
}

impl<T> State<T> {
    fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

type GamesState = State<OwnedGames>;
type WinsState = State<u32>;

#[tokio::main]
async fn main() {
    let games_state = GamesState::new();
    let wins_state = WinsState::new();

    tracing_subscriber::fmt()
        // .with_max_level(tracing::Level::TRACE)
        .with_env_filter("info,stream_api=debug,tower_http=debug,reqwest_retry=trace")
        .init();

    // Compose the routes
    let app = Router::new()
        .route("/", get(|| async { "Hello from Stream API!" }))
        .route("/sentence/{*name}", get(sentence::get_sentence))
        .route(
            "/steam/{steamid}/{appid}/hours",
            get(steam::get_hours_played)
                .layer(Extension(games_state))
                .layer(CacheLayer::with_lifespan(60 * 60)),
        )
        .route(
            "/youtube/{channel}/video",
            get(youtube::get_last_video).layer(CacheLayer::with_lifespan(60)),
        )
        .route(
            "/youtube/{channel}/short",
            get(youtube::get_last_short).layer(CacheLayer::with_lifespan(60)),
        )
        .route(
            "/win/{key}/{command}",
            get(win::get_win).layer(Extension(wins_state)),
        )
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(60))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
