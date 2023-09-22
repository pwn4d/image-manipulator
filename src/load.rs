extern crate image;
use image::{DynamicImage, GenericImage, ImageFormat, ImageError};

pub fn load_image(image_path: &str) -> Result<DynamicImage, ImageError> {
    return image::open(image_path)
}