use crate::meow::meow::*;
use std::fs::File;

pub fn read_file(path: String) -> String {
   
    match File::open(file_path) {
        Ok(file) => {
            use std::io::Read;
            let mut reader = std::io::BufReader::new(file);
            let mut content = String::new();
            reader
                .read_to_string(&mut content)
                .expect("Failed to read file");
            content
        }
        Err(_) => String::from("Error reading the JSONL file"),
    }
}
