use chrono::prelude::*;
use chrono::{Local, DateTime, TimeZone};

use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn create_log_entry() {

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