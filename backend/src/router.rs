use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::tools::image::convert;

const MAX_BODY_SIZE: usize = 1024 * 1024 * 10;
pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async {}))
        .route("/api/health", get(health))
        .route("/api/image/convert", post(convert))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(DefaultBodyLimit::max(MAX_BODY_SIZE)),
        )
}
async fn health() -> &'static str {
    "Healthy"
}
