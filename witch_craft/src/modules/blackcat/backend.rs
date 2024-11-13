use crate::core::core::*;
use sha256::try_digest;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

pub fn malware_scanner(path: String) -> Vec<(String, String)> {
    let malware_signatures: Vec<String> =
        match File::open(&get_witch_spells_path("malware/full.csv")) {
            Ok(value) => {
                let lines = io::BufReader::new(value).lines();
                lines.collect::<Result<Vec<_>, _>>().unwrap_or_else(|err| {
                    raise(&format!("malware_scanner :: {}", err.to_string()), "error");
                    Vec::new()
                })
            }
            Err(err) => {
                raise(&format!("malware_scanner :: {}", err.to_string()), "error");
                Vec::new()
            }
        };

    let metadata = match fs::metadata(&path) {
        Ok(value) => value,
        Err(err) => {
            raise(
                &format!(
                    "malware_scanner :: invalid file or folder path :: {}",
                    err.to_string()
                ),
                "error",
            );
            exit(1);
        }
    };
    let mut malware_found = Vec::new();

    if metadata.is_file() {
        let file_path = Path::new(&path);
        let file_sig = try_digest(file_path).unwrap();
        for sig in &malware_signatures {
            if sig.contains(&file_sig) {
                malware_found.push((sig.to_string(), file_path.to_string_lossy().to_string()));
            }
        }
    }

    if metadata.is_dir() {
        let fs_path = Path::new(&path);
        let files = directory_lookup(fs_path);
        for file in files {
            let file_sig = try_digest(&file).unwrap();
            for sig in &malware_signatures {
                if sig.contains(&file_sig) {
                    malware_found.push((sig.to_string(), file.to_string()));
                }
            }
        }
    }

    malware_found
}

/// Scans a file or folder from a given path, and takes an action (currently only "remove") based on user input.
///
/// This function processes command-line arguments to scan the provided path. The path can refer to either a single file
/// or a folder. If the path points to a folder, the function will recursively scan through all files within the folder.
/// The only available action at the moment is "remove", which will delete the file or folder and its contents (if recursive).
///
/// If an invalid action is provided, no action is taken.
///
/// # Arguments
///
/// * `argsv` - A slice of `String` references representing the command line arguments.
///
/// # Returns
///
/// * `i32` - Returns `0` on success or `1` if there was an error (e.g., missing argument, invalid path, or failed action).
///
/// # Behavior
///
/// - If the path is a file, the function will take the specified action directly on that file.
/// - If the path is a folder, it will recursively scan through the folder and apply the action to each file and sub-folder.
/// - Only the "remove" action is supported, which will delete the file or recursively delete the contents of the folder.
///
/// # Example
///
/// ```
/// let args = vec!["blackcat_av".to_string(), "--path".to_string(), "/path/to/scan".to_string(), "--action".to_string(), "remove".to_string()];
/// blackcat_av(&args);
/// ```
///
/// In this example, if the path is valid, the function will recursively delete the folder or remove the file at the given path.
///
/// Note: Be cautious when using the "remove" action, as it permanently deletes files.
pub fn blackcat_av(argsv: &[String]) -> i32 {
    let path = search_value("path", argsv);
    let action = search_value("action", argsv);

    let malware_result = malware_scanner(path.clone());

    if malware_result.is_empty() {
        raise("Nothing found! :: System may be clean", "done");
        return 0;
    }

    fn info(path: &String, malware_result: &Vec<(String, String)>) {
        for sig in malware_result {
            raise(&format!("Malware found at :: {}", path), "warning");
            let header = [
                "first seen utc",
                "hash sha256",
                "hash md5",
                "hash sha1",
                "reporter",
                "file name",
                "file type guess",
                "mime type",
                "signature",
                "clamav",
                "vtpercent",
                "imphash",
                "ssdeep",
                "tlsh",
            ];
            let data = sig.0.replace("\"", "");
            let info: Vec<&str> = data.split(",").collect();
            for (head, body) in header.iter().zip(info.iter()) {
                println!("\t{}: {:?}", head, body.trim());
            }
        }
        println!("");
    }

    let mut done: Vec<String> = Vec::new();
    let mut gone: Vec<String> = Vec::new();

    if action != "remove" {
        raise(
            "Malware found! RUN this command with --action remove",
            "warning",
        );
    }

    for mal in &malware_result {
        info(&mal.1, &malware_result);
        if action == "remove" {
            let path = Path::new(&mal.1);
            match fs::remove_file(path) {
                Ok(_) => done.push(path.to_string_lossy().to_string()),
                Err(err) => {
                    raise(&format!("blackcat_av :: {}", err.to_string()), "fail");
                    gone.push(path.to_string_lossy().to_string())
                }
            }
        }
    }

    for dn in done {
        let msg = format!("Malware removed :: {}", dn);
        raise(&msg, "done");
    }

    for gn in &gone {
        let msg = format!("Malware found but not removed :: {}", gn);
        raise(&msg, "warning");
    }

    if gone.is_empty() {
        return 0;
    }

    255
}

pub fn blackcat_chkrootkit_bind(_args: &[String]) -> i32 {
    raise("Running scan it can take a while...", "message");
    lazy_exec("sudo chkrootkit".to_string())
}
