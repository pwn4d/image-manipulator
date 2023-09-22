extern crate image;
mod flip;
mod invert;
mod load;
use std::env;
use image::DynamicImage;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    let input_name = &arguments[1].trim().to_string();
    let output_name = &arguments[2].trim().to_string();
    let invert = &arguments[3].trim().to_string();
    let mut flip_type_str = &arguments[3];
    let invert_enabled = false;
    let flip_enabled = false;





    let flip_type = match flip_type_str.to_lowercase().as_str() {   
        "vertical" => flip::FlipType::Vertical,
        "v" => flip::FlipType::Vertical,
        "horizontal" => flip::FlipType::Horizontal,
        "h" => flip::FlipType::Horizontal,
        _ => {
            eprintln!("Invalid flip type");
                return;
        }
    };




    let mut image = match load::load_image(input_name) {
        Ok(image) => image,
        Err(err) => panic!("Loading image failed: {:?}", err),
    };


    match flip::flip_image(&image,flip_type) {
        Ok(flipped_image) => {
            println!("Image Flip Success");
            image = flipped_image; // Update the image with the flipped image
        }
        Err(err) => eprintln!("{:?}", err),
    }

    match invert::invert_image(&mut image) {

    Ok(_) => {
        println!("Image Inversion Success")
    }
    Err(err) => eprintln!("{:?}", err)} ;
    
    image.save(output_name).unwrap()
    





    
}

