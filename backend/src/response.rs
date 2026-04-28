use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use thiserror::Error;

// Error response
#[derive(Debug, Error)]
pub enum AppError {
    // Server errors
    #[error("Not found")]
    NotFound,
    #[error("Internal error")]
    InternalError,
    // IO Errors
    #[error("Payload too large")]
    PayloadTooLarge,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND", "Resource not found".to_string()),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR", "Internal server error".to_string()),
            AppError::PayloadTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, "PAYLOAD_TOO_LARGE", "Payload too large".to_string()),
            AppError::InvalidInput(message) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", format!("Invalid input: {}", message))
        };
        return error_res(status, code, message);
    }
}
pub fn error_res(status: StatusCode, code: &str, message: String) -> Response {
    let res = json!({
        "ok": false,
        "data" : null,
        "error" : {
            "code" : code,
            "message": message,
        }
    });
    (status, Json(res)).into_response()
}
// Success response
pub fn success_res<T: Serialize>(data: T) -> Json<Value> {
    Json(json!({
        "ok": true,
        "data": data,
        "error": null
    }))
}