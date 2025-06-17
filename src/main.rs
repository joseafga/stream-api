use axum::{Router, error_handling::HandleErrorLayer, http::StatusCode, routing::get};
use axum_response_cache::CacheLayer;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod sentence;
mod steam;
mod youtube;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Compose the routes
    let app = Router::new()
        .route("/", get(|| async { "Hello from Stream API!" }))
        .route("/sentence/{*name}", get(sentence::get_sentence))
        .route(
            "/steam/{steamid}/{appid}/hours",
            get(steam::get_hours_played).layer(CacheLayer::with_lifespan(60 * 60)),
        )
        .route(
            "/youtube/{channel}/video",
            get(youtube::get_last_video).layer(CacheLayer::with_lifespan(60)),
        )
        .route(
            "/youtube/{channel}/short",
            get(youtube::get_last_short).layer(CacheLayer::with_lifespan(60)),
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
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
