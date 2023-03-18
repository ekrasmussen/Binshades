extern crate image;

mod imgedit;
use image::{ImageBuffer};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const BLOCK_SIZE: u32 = 12;

const GRIDX: u32 = WIDTH / BLOCK_SIZE;
const GRIDY: u32 = HEIGHT / BLOCK_SIZE;



fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        println!("File: {}", args[i]);

        let binary_data = match convert_to_binary(&args[i]) {
            Ok(data) => data,
            Err(e) => {
                println!("Error loading file, check file path again");
                return;
            }
        };
        
        println!("Buffer: ");
        println!("{:?}", binary_data);

        //convert_to_text("result.txt", &binary_data).unwrap();

        let mut image = ImageBuffer::new(WIDTH, HEIGHT);
        image = imgedit::fill_image(image, image::Luma([255u8]));
        image.save("output.png").unwrap();
    }
}

fn convert_to_binary(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn convert_to_text(path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}