use serde::Deserialize;

mod convert;
pub use convert::convert;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Jpg,
    Webp,
    Bmp,
}