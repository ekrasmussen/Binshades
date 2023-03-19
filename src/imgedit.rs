use image::{ImageBuffer};
use std::fs::create_dir_all;
use std::path::Path;
use image::Rgb;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const BLOCK_SIZE: usize = 12;

const GRIDX: usize = WIDTH / BLOCK_SIZE;
const GRIDY: usize = HEIGHT / BLOCK_SIZE;

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

pub fn create_image_colored(colors: [[[u8; 3]; GRIDY]; GRIDX], id: usize, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
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