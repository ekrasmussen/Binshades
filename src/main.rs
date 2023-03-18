extern crate image;

mod imgedit;
use image::{ImageBuffer};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

const WIDTH: usize = 48;
const HEIGHT: usize = 24;
const BLOCK_SIZE: usize = 12;

const GRIDX: usize = WIDTH / BLOCK_SIZE;
const GRIDY: usize = HEIGHT / BLOCK_SIZE;



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
        println!("{}", binary_data.len());

        let mut theVector = generate_image_filestream(binary_data);
        println!("Total amount of images: {}", theVector.len());

        print_image_as_text(&theVector[1]);

        let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
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

fn generate_image_filestream(mut data: Vec<u8>) -> Vec<[[u8; GRIDY]; GRIDX]> {
    let mut arrays = Vec::new();
    let mut index = 0;
    while index < data.len() {
        let mut array = [[255; GRIDY]; GRIDX];
        for y in 0..GRIDY {
            for x in 0..GRIDX {
                if index < data.len() {
                    array[x][y] = data[index];
                    index += 1;
                } else {
                    for y in x..GRIDY {
                        for x in y..GRIDX {
                            array[x][y] = 255;
                        }
                    }
                    arrays.push(array);
                    return arrays;
                }
            }
        }
        arrays.push(array);
    }
    arrays
}

fn print_image_as_text(array: &[[u8; GRIDY]; GRIDX]) {
    for i in 0..array[0].len() {
    for j in 0..array.len(){
        print!("{:3} ", array[j][i]);
    }
    println!();
}
}