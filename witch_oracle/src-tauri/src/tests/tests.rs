use fs::File;
use mockall::prelude::*;
use std::env;
use std::fs;

#[test]
fn readrc_value_empty_file() {
    // Create an empty file at HOME/.witchrc
    fs::create_dir_all(env::var("HOME").unwrap());
    let mut file = File::create(format!("/{}", env::var("HOME")).as_path()).unwrap();
    let _ = file.write(b"").unwrap();

    assert_eq!(readrc_value("test"), "error");
}

#[test]
fn readrc_value_found_line() {
    // Create a file at HOME/.witchrc with the key
    fs::create_dir_all(env::var("HOME").unwrap());
    let mut file = File::create(format!("/{}", env::var("HOME")).as_path()).unwrap();
    let data = b"test_key=test_value\n";
    let _ = file.write(data).unwrap();

    assert_eq!(readrc_value("test"), "test_value");
}

#[test]
fn readrc_value_not_found() {
    // Create a directory at HOME/.witchrc
    fs::create_dir_all(env::var("HOME").unwrap());

    assert_eq!(readrc_value("test"), "".to_string());
}

#[mockall::mock]
impl fs for () {
    fn read_to_string<P>(&self, path: P) -> Result<String, std::io::Error>
    where
        P: AsRef<std::path::Path>,
    {
        match path.as_ref().file_name() {
            Some(file_name) => {
                if *file_name == "test_file.txt" {
                    return Ok("mock_value".to_string());
                }
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                ));
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            )),
        }
    }
}

#[test]
fn read_json() {
    // Set the path_log_file key in HOME/.witchrc
    let mut file = File::create(format!("/{}", env::var("HOME")).as_path()).unwrap();
    let data = b"{}=path_log_file=path_to_file.txt\n".to_vec();
    let _ = file.write(data).unwrap();

    // Assert that the read_json command returns "mock_value"
    assert_eq!(read_json(), "mock_value");
}
