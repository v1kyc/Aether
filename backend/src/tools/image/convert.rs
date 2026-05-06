use std::io::Cursor;
use axum::{
    extract::{Multipart, multipart::Field},
    response::Response,
};
use crate::{
    rxtx::{AppError, multipart::extract},
    tools::image::ImageFormat,
};
use serde::Deserialize;
use image::ImageReader;

#[derive(Deserialize)]
pub struct ImageConvertOptions {
    pub target: ImageFormat,
}
pub async fn convert(mut multipart: Multipart) -> Result<Response, AppError> {

    let (opts, file): (ImageConvertOptions, Field) = extract(&mut multipart).await?;

    // Image
    let bytes = file.bytes().await
        .map_err(|_| AppError::InvalidInput("Failed to read image".to_string()))?;

    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|_| AppError::InvalidInput("Failed to guess format".to_string()))?
        .decode()
        .map_err(|_| AppError::InvalidInput("Failed to decode image".to_string()))?;

    img.save("recieved.jpg").map_err(|_| AppError::InvalidInput("Failed to save image".to_string()))?;
    todo!()
}
