extern crate image;
use image::{DynamicImage, GenericImage, ImageFormat, ImageError};

pub fn loadImage(image_path: &str) -> Result<DynamicImage, ImageError> {
    match image::open(image_path) {

        Ok(image) => return Ok(image),
        Err(_) => return Err(ImageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Invalid Image!")))
    };
}