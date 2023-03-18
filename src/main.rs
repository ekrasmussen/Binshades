extern crate image;

use image::{ImageBuffer};
use std::env;
use std::fs::File;
use std::io::Read;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const BLOCK_SIZE: u32 = 12;

const GRIDX: u32 = WIDTH / SIZE;
const GRIDY: u32 = HEIGHT / SIZE;

fn main() {
    println!("Hello, world!");
}
