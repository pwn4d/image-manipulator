extern crate image;
use image::imageops::colorops::invert;
use image::{DynamicImage, GenericImage, ImageFormat, ImageError};


pub fn invert_image(image: &mut DynamicImage) -> Result<(), ImageError>{
    invert(image);
    Ok(())
   
}
