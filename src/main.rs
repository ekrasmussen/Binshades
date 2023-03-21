extern crate image;

mod imgedit;
use image::{ImageBuffer};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Instant;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const BLOCK_SIZE: usize = 12;
const THREADS: usize = 4;

const GRIDX: usize = WIDTH / BLOCK_SIZE;
const GRIDY: usize = HEIGHT / BLOCK_SIZE;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args[1] == "test" {
        let test_files = vec!["50KB.bin", "100KB.bin", "250KB.bin", "500KB.bin", "750KB.bin", "1MB.bin", "2MB.bin", "4MB.bin", "5MB.bin", "10MB.bin", "20MB.bin", "40MB.bin", "50MB.bin", "100MB.bin", "250MB.bin", "500MB.bin" "1GB.bin"];


    }

    for i in 1..args.len() {
        println!("File: {}", args[i]);

        //Convert to binary
        let binary_data = match convert_to_binary(&args[i]) {
            Ok(data) => data,
            Err(e) => {
                println!("Error loading file, check file path again");
                return;
            }
        };

        let mut image_values = imgedit::generate_image_filestream_colored(binary_data);

        let chunk_size = image_values.len() / THREADS;
        let remainder = image_values.len() % THREADS;

        let mut threads = vec![];

        let mut start = 0;
        for i in 0..THREADS {
            let mut end = start + chunk_size;
            if i == THREADS - 1 {
                end += remainder;
            }

            let values_chunk = image_values[start..end].to_vec();
            let thread_handle = thread::spawn(move || {
                for (idx, value) in values_chunk.iter().enumerate() {
                    let image_index = start + idx;
                    imgedit::create_image_colored(*value, image_index);
                }
            });

            threads.push(thread_handle);

            start = end;
        }


        //waiting for threads to finish
        for thread_handle in threads {
            thread_handle.join().unwrap();
        }
        // for i in 0..image_values.len() {
        //     imgedit::create_image_colored(image_values[i], i);
        // }
        println!("Total amount of images: {}", image_values.len());
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn create_directory(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
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

fn print_image_as_text(array: &[[u8; GRIDY]; GRIDX]) {
    for i in 0..array[0].len() {
    for j in 0..array.len(){
        print!("{:3} ", array[j][i]);
    }
    println!();
}
}