use axum::Router;
use axum::routing::get;

pub fn router () -> Router {
    Router::new()
        .route("/", get(|| async {}))
        .route("/health", get(health))
}
async fn health () -> &'static str {"Healthy"}