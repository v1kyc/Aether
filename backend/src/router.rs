use axum::extract::DefaultBodyLimit;
use axum::Router;
use axum::routing::{get, post};
use tower_http::{trace::TraceLayer,cors::CorsLayer,};
use tower::ServiceBuilder;

const MAX_BODY_SIZE: usize = 1024 * 1024 * 10;
pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async {}))
        .route("/health", get(health))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(DefaultBodyLimit::max(MAX_BODY_SIZE))
        )
}
async fn health() -> &'static str {
    "Healthy"
}
