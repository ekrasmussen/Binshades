use image::{ImageBuffer};
use std::fs::create_dir_all;
use std::path::Path;


const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const BLOCK_SIZE: usize = 12;

const GRIDX: usize = WIDTH / BLOCK_SIZE;
const GRIDY: usize = HEIGHT / BLOCK_SIZE;

pub fn fill_image(mut image: ImageBuffer<image::Luma<u8>, Vec<u8>>, color: image::Luma<u8>) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();

    for i in 0..width {
        for j in 0..height {
            image.put_pixel(i, j, color);
        }
    }

    image
}

pub fn generate_image_filestream(mut data: Vec<u8>) -> Vec<[[u8; GRIDY]; GRIDX]> {
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
                    for y in y..GRIDY {
                        for x in x..GRIDX {
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

pub fn generate_image_filestream_colored(mut data: Vec<u8>) -> Vec<[[[u8; 3]; GRIDY]; GRIDX]> {
    let mut arrays = Vec::new();
    let mut index = 0;
    while index < data.len() {
        let mut array = [[[255; 3]; GRIDY]; GRIDX];
        for y in 0..GRIDY {
            for x in 0..GRIDX {
                for c in 0..3 {
                    if index < data.len() {
                        array[x][y][c] = data[index];
                        index += 1;
                    } else {
                        for y in y..GRIDY {
                            for x in x..GRIDX {
                                for c in c..3 {
                                    array[x][y][c] = 255;
                                }
                            }
                        }

                        arrays.push(array);
                        return arrays;
                    }
                }
            }
        }
        arrays.push(array);
    }
    arrays
}

pub fn create_image(colors: [[u8; GRIDY]; GRIDX], id: usize)
{
    //Create the subdirectory
    let dir_path = Path::new("output");
    create_dir_all(dir_path);
    
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let filename = format!("outputbw{}.png", id.to_string());
    let image_path = dir_path.join(filename);

    println!("GRIDX: {}", GRIDX);
    println!("GRIDY: {}", GRIDY);
    for x in 0..colors.len() {
        for y in 0..colors[0].len() {
            let startx = x * BLOCK_SIZE;
            let starty = y * BLOCK_SIZE;

            for dx in 0..BLOCK_SIZE {
                for dy in 0..BLOCK_SIZE {
                    image.put_pixel(dx as u32 + startx as u32, dy as u32 + starty as u32, image::Luma([colors[x][y]]));
                }
            }
        }
    }

    image.save(&image_path).unwrap();
}

pub fn create_image_colored(colors: [[[u8; 3]; GRIDY]; GRIDX], id: usize) {
    //Create the subdirectory
    let dir_path = Path::new("output");
    create_dir_all(dir_path);
    
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let filename = format!("outputcolor{}.png", id.to_string());
    let image_path = dir_path.join(filename);
    println!("GRIDX: {}", GRIDX);
    println!("GRIDY: {}", GRIDY);
    for x in 0..colors.len() {
        for y in 0..colors[0].len() {
            let startx = x * BLOCK_SIZE;
            let starty = y * BLOCK_SIZE;

            for dx in 0..BLOCK_SIZE {
                for dy in 0..BLOCK_SIZE {
                    image.put_pixel(dx as u32 + startx as u32, dy as u32 + starty as u32, image::Rgb([colors[x][y][0], colors[x][y][1], colors[x][y][2]]));
                }
            }
        }
    }

    image.save(&image_path).unwrap();
}