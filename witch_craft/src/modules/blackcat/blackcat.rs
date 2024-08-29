use crate::modules::core::core::*;
use sha256::try_digest;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn malware_scanner(path: String) -> Vec<String> {
    let malware_signatures: String =
        match fs::read_to_string("/var/witch_craft/witch_spells/malware/malware.list") {
            Ok(value) => value,
            Err(err) => {
                raise(&format!("Error at {}", err), 0);
                String::new()
            }
        };

    let metadata = fs::metadata(&path).unwrap();
    let mut malware_found = Vec::new();

    if metadata.is_file() {
        let file_path = Path::new(&path);
        let file_sig = try_digest(file_path).unwrap();
        if malware_signatures.contains(&file_sig) {
            malware_found.push(file_path.to_string_lossy().to_string());
        }
    }

    if metadata.is_dir() {
        let fs_path = Path::new(&path);
        let files = directory_lookup(fs_path);
        for file in files {
            let file_sig = try_digest(&file).unwrap();
            if malware_signatures.contains(&file_sig) {
                malware_found.push(file);
            }
        }
    }

    malware_found
}

pub fn blackcat_av(argsv: &[String]) -> i32 {
    let path = search_value("path", argsv);
    let action = search_value("action", argsv);

    let malware_result = malware_scanner(path.clone());

    if malware_result.is_empty() {
        raise("Nothing found! :: System may be clean", 6);
        return 0;
    }

    let mut done: Vec<String> = Vec::new();
    let mut gone: Vec<String> = Vec::new();

    if action != "remove" {
        let msg = format!(
            "Malware found! RUN this command with --action remove ::\nLocation :: {} ",
            &path
        );
        raise(&msg, 6);
        return 0;
    }

    for mal in malware_result {
        let path = Path::new(&mal);
        match fs::remove_file(path) {
            Ok(_) => done.push(path.to_string_lossy().to_string()),
            Err(err) => {
                println!("{}", err);
                gone.push(path.to_string_lossy().to_string())
            }
        }
    }

    for dn in done {
        let msg = format!("Malware removed :: {}", dn);
        raise(&msg, 1);
    }

    for gn in &gone {
        let msg = format!("Malware found but not removed :: {}", gn);
        raise(&msg, 2);
    }

    if gone.is_empty() {
        return 0;
    }

    255
}
