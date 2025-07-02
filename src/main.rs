use axum::{Extension, Router, error_handling::HandleErrorLayer, http::StatusCode, routing::get};
use axum_response_cache::CacheLayer;
use std::{sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

pub mod cache;
mod sentence;
mod steam;
mod win;
mod youtube;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100);
    let games_state = cache::GamesState::new();
    let wins_state = Arc::new(cache::WinsState {
        cache: cache::Cache::new(),
        sender: tx,
    });

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
            get(win::get_win).with_state(wins_state.clone()),
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
