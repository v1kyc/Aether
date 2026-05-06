use thiserror::Error;

pub mod multipart;
pub mod response;

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
