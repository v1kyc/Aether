use crate::response::{AppError, success_res};
use axum::Json;
use axum::response::Response;
use bcrypt::{hash, verify};
use serde::Deserialize;

const MIN_COST: u32 = 4;
const MAX_COST: u32 = 31;
const MAX_PASSWORD_LEN: usize = 72; // bcrypt silently truncates at 72 bytes

// Logic
fn bcrypt_hash<T: AsRef<[u8]>>(data: T, cost: u32) -> Result<String, AppError> {
    hash(data, cost).map_err(|_| AppError::InternalError)
}

fn bcrypt_verify<T: AsRef<[u8]>>(data: T, hashed: &str) -> Result<bool, AppError> {
    verify(data, hashed).map_err(|_| AppError::InvalidInput("Invalid hash format".to_string()))
}

// Handlers
#[derive(Deserialize)]
pub struct HashRequest {
    pub password: String,
    pub cost: u32,
}
#[derive(Deserialize)]
pub struct VerifyRequest {
    pub password: String,
    pub hash: String,
}

pub async fn hash_handler(Json(body): Json<HashRequest>) -> Result<Response, AppError> {
    if body.password.is_empty() {
        return Err(AppError::InvalidInput(
            "Password cannot be empty".to_string(),
        ));
    }
    if body.password.len() > MAX_PASSWORD_LEN {
        return Err(AppError::InvalidInput(format!(
            "Password cannot exceed {} bytes",
            MAX_PASSWORD_LEN
        )));
    }
    if body.cost < MIN_COST || body.cost > MAX_COST {
        return Err(AppError::InvalidInput(format!(
            "Cost must be between {} and {}",
            MIN_COST, MAX_COST
        )));
    }
    let hashed = bcrypt_hash(&body.password, body.cost)?;
    Ok(success_res(hashed))
}

pub async fn verify_handler(Json(body): Json<VerifyRequest>) -> Result<Response, AppError> {
    if body.password.is_empty() || body.hash.is_empty() {
        return Err(AppError::InvalidInput(
            "Password and hash cannot be empty".to_string(),
        ));
    }
    if body.password.len() > MAX_PASSWORD_LEN {
        return Err(AppError::InvalidInput(format!(
            "Password cannot exceed {} bytes",
            MAX_PASSWORD_LEN
        )));
    }
    let valid = bcrypt_verify(&body.password, &body.hash)?;
    Ok(success_res(valid))
}
