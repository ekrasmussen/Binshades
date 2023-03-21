use chrono::prelude::*;
use chrono::{Local, DateTime, TimeZone};

use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::time::Duration;

pub fn create_log_entry() {
    let text = format!("####################\nEntry: {}\n####################\n", get_current_time());
    write_log(&text);
}

pub fn add_benchmark(filename: &str, filestream: Duration, image: Duration, total: Duration) {
    let text = format!("Data for file: {}\n    Filestream time: {:.2}\n    Image Write Time: {:.2}\n    Total Time: {:.2}\n\n", filename, filestream.as_secs_f64(), image.as_secs_f64(), total.as_secs_f64());
    write_log(&text);
}

pub fn finalize_log(total_time: Duration) {
    let text = format!("Benchmark over, total time was {:.2}\n\n", total_time.as_secs_f64());
    write_log(&text);
}

pub fn write_log(content: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn get_current_time() -> String {
    let local_time = Local::now();
    let formatted_time = local_time.format("%d/%m/%Y %H:%M").to_string();

    formatted_time
}