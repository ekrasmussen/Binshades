extern crate image;

mod imgedit;
use image::{ImageBuffer};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::thread;

const WIDTH: usize = 3840;
const HEIGHT: usize = 2160;
const BLOCK_SIZE: usize = 12;
const THREADS: usize = 4;

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
        
        // println!("Buffer: ");
        // println!("{:?}", binary_data);
        // println!("{}", binary_data.len());

        let mut image_values = generate_image_filestream_colored(binary_data);
        println!("Total amount of images: {}", image_values.len());

        //print_image_as_text(&image_values[0]);

        // let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        // image = imgedit::fill_image(image, image::Luma([255u8]));
        // image.save("output.png").unwrap();

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

            // let thread_handle = thread::spawn(move ||{ 
            //     for value in values_chunk {
            //         imgedit::create_image_colored(value, start);
            //     }
            // });
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

fn generate_image_filestream_colored(mut data: Vec<u8>) -> Vec<[[[u8; 3]; GRIDY]; GRIDX]> {
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

fn print_image_as_text(array: &[[u8; GRIDY]; GRIDX]) {
    for i in 0..array[0].len() {
    for j in 0..array.len(){
        print!("{:3} ", array[j][i]);
    }
    println!();
}
}