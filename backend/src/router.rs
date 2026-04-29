use crate::tools::hashing::bcrypt::{hash_handler, verify_handler};
use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async {}))
        .route("/health", get(health))
        .route("/tools/hashing/bcrypt/hash", post(hash_handler))
        .route("/tools/hashing/bcrypt/verify", post(verify_handler))
}
async fn health() -> &'static str {
    "Healthy"
}
