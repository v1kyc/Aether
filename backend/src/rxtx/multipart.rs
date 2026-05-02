use crate::rxtx::response::AppError;
use axum::extract::Multipart;
use axum::extract::multipart::Field;
use serde::de::DeserializeOwned;

async fn extract<T: DeserializeOwned>(multipart: &'_ mut Multipart) -> Result<(T, Field<'_>), AppError> {
    let options = multipart
        .next_field()
        .await
        .map_err(|_| AppError::InvalidInput("Failed to read field".to_string()))?;

    if let Some(field) = options {
        todo!()
    } else {
        return Err(AppError::InvalidInput("Missing options field".to_string()));
    }
}
