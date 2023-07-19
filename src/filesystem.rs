use std::fs;
use std::fs::{OpenOptions,File};
use std::io::{self, prelude::*};

pub fn open_file(file_path : &str) -> Result<(),io::Error> {
    File::open(file_path)?;
    Ok(())
}

pub fn create_file(file_path : &str) -> Result<(),io::Error> {
    File::create(file_path)?;
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

pub fn delete_file(file_path : &str) -> Result<(),io::Error> {
    fs::remove_file(file_path)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let file_path = "test_file.txt";
        assert!(create_file(file_path).is_ok());

        // Clean up the file after the test is finished.
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_open_file() {
        let file_path = "test_file.txt";
        create_file(file_path).unwrap();
        assert!(open_file(file_path).is_ok());

        // Clean up the file after the test is finished.
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_file() {
        let file_path = "test_file.txt";
        let test_data = "hello world";
        create_file(file_path).unwrap();
        write_to_file(file_path, test_data).unwrap();
        assert_eq!(read_file(file_path).unwrap(),test_data);
        
        // Clean up the file after the test is finished.
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_write_to_file(){
        let file_path = "test_file.txt";
        let test_data = "hello world";
        create_file(file_path).unwrap();
        assert!(write_to_file(file_path, test_data).is_ok());
        
        // Clean up the file after the test is finished.
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_delete_file() {
        let file_path = "test_file.txt";
        create_file(file_path).unwrap();
        assert!(delete_file(file_path).is_ok());
    }
    
}