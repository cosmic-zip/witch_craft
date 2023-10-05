use crate::meow::meow::*;
use std::fs::File;

pub fn read_jsonl_file() -> String {
    // Replace "your_jsonl_file.jsonl" with the path to your JSONL file.
    let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let file_path = format!("{}{}", config["REPORT_BASE_PATH"], config["REPORT_LOG"]);


    match File::open(file_path) {
        Ok(file) => {
            use std::io::Read;
            let mut reader = std::io::BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content).expect("Failed to read file");
            content
        }
        Err(_) => String::from("Error reading the JSONL file"),
    }
}
