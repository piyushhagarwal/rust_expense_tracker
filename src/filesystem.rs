use std::fs::{OpenOptions,File};
use std::io::{self, prelude::*};

pub fn create_file(file_path : &str) -> Result<(),io::Error> {
    let _file = File::create(file_path)?;
    Ok(())
}

pub fn read_file(file_path : &str) -> Result<String,io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_to_file(file_path : &str, data : &str) -> Result<(),io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}