use image::{DynamicImage, ImageError};

pub fn flipImage(image: &DynamicImage, flip_type: &str) -> Result<DynamicImage, ImageError> {
    let flipped_image = if flip_type == "horizontal" {
        image.fliph()
    } else if flip_type == "vertical" {
        image.flipv()
    } else {
        return Err(ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Please Enter horizontal or vertical",
        )));
    };
    Ok(flipped_image)
}