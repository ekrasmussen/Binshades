use image::{ImageBuffer};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const BLOCK_SIZE: usize = 48;

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

pub fn create_image(colors: [[u8; GRIDY]; GRIDX], id: usize)
{
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let filename = format!("output{}.png", id.to_string());
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
            //println!("Printed a field!");
        }
    }

    image.save(filename).unwrap();
}