use serde::Deserialize;

mod convert;
pub use convert::convert;

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Jpg,
    Webp,
    Bmp,
}
impl From<ImageFormat> for image::ImageFormat {
    fn from(f: ImageFormat) -> image::ImageFormat {
        match f {
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpg => image::ImageFormat::Jpeg,
            ImageFormat::Webp => image::ImageFormat::WebP,
            ImageFormat::Bmp => image::ImageFormat::Bmp,
        }
    }
}
impl ImageFormat {
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpg => "image/jpeg",
            ImageFormat::Webp => "image/webp",
            ImageFormat::Bmp => "image/bmp",
        }
    }
}