extern crate image;
mod flip;
mod invert;
mod load;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let input_name = &arguments[1].trim().to_string();
    let output_name = &arguments[2].trim().to_string();
   

    let mut image = load::loadImage(input_name).unwrap();

    match flip::flipImage(&image, "vertical") {
        Ok(flipped_image) => {
            println!("Image Flip Success");
            image = flipped_image; // Update the image with the flipped image
        }
        Err(err) => eprintln!("{:?}", err),
    }

    match invert::invertImage(&mut image) {

    Ok(_) => {
        println!("Image Inversion Success")
    }
    Err(err) => eprintln!("{:?}", err)} ;
    
    image.save(output_name).unwrap()
    





    
}

