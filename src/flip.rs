use image::{DynamicImage, ImageError};
#[derive(Eq, PartialEq)]
pub enum FlipType {
    Vertical,
    Horizontal,
}
pub fn flip_image(image: &DynamicImage, flip_type: FlipType) -> Result<DynamicImage, ImageError> {
    let flipped_image = if flip_type == FlipType::Horizontal {
        image.fliph()
    } else if flip_type == FlipType::Vertical {
        image.flipv()
    } else {
        return Err(ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Please Enter horizontal or vertical",
        )));
    };
    Ok(flipped_image)
}