use crate::core::utils::*;
use crate::meow::meow::read_meow;
use crate::modules::maid_av::entropy::advanced_entropy_scanner;
use sha256::{digest, try_digest};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn search_malware_hash(search_term: &str, debug: bool) -> bool {
    let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let malware_db = &format!("{}{}", config["GENRAL_BASE_PATH"], config["MALWARE_HASH"]);

    let file_path = malware_db;

    if debug == true {
        system_text(&format!("🔴 [ERROR] :: file path {}", file_path), "red");
        println!("{}", file_path);
    }

    match search_csv(file_path, search_term) {
        Ok(matches) => {
            if !matches.is_empty() {
                println!("Matching Rows:");
                for row in matches {
                    println!(
                        "🚧 [MALWARE_SCANNING] :: Warning malware was being found :: {}",
                        row
                    );
                }
                return true;
            } else {
                system_text("🔴 [WARNING] :: Pattern not found in any line.", "red");
                return false;
            }
        }
        Err(err) => {
            eprintln!("🔴 Error: {}", err);
            return false;
        }
    }
}

pub fn search_malware_pattern(pattern: &str, debug: bool) -> bool {
    let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let malware_db = &format!("{}{}", config["GENRAL_BASE_PATH"], config["MALWARE_HASH"]);

    system_text(
        "⚪ [MALWARE_PATTERN] :: Searching for malware pattern",
        "green",
    );

    match find_all_matching_lines(malware_db, pattern) {
        Ok(result) => {
            if !result.is_empty() {
                for line in result {
                    println!("🚧 {}", line);
                }
                return true;
            } else {
                println!("⚪ [WARNING] :: Pattern not found in any line.");
                return true;
            }
        }
        Err(err) => {
            eprintln!("🔴 [ERROR] :: {}", err);
            return false;
        }
    }
}

pub fn calculate_sha256_hash(file_path: &str, debug: bool) -> Result<String, io::Error> {
    let input = Path::new(file_path);
    let hash = try_digest(input).unwrap();

    if debug == true {
        println!("🚧 [DEBUG] :: sha256 :{}", hash);
        println!("🚧 [DEBUG] :: file_path :{}", file_path);
    }

    Ok(hash)
}

pub fn list_files_and_folders(dir_path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut items = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        items.push(path.clone());

        if path.is_dir() {
            let sub_items = list_files_and_folders(path.to_str().unwrap())?;
            items.extend(sub_items);
        }
    }

    Ok(items)
}

pub fn active_malware_scanner(derectory: &str, debug: bool) -> bool {
    if debug == true {
        println!("{}", derectory);
    }

    match list_files_and_folders(derectory) {
        Ok(items) => {
            for item in items {
                match calculate_sha256_hash("/bin/yes", true) {
                    Ok(result) => {
                        return search_malware_pattern(&result, debug);
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        return false;
                    }
                }
            }
            return true;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return false;
        }
    }
}

pub fn shell_maid_av(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--hash" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = &gsv(system_input.clone(), "--hash");

            search_malware_hash(instance, debug)
        }

        "--pattern" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = &gsv(system_input.clone(), "--pattern");

            search_malware_pattern(instance, debug)
        }

        "--scanner" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = &gsv(system_input.clone(), "--directory");

            active_malware_scanner(instance, debug)
        }

        "--entropy" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = &gsv(system_input.clone(), "--path");
            advanced_entropy_scanner(instance, debug)
        }

        _ => {
            system_text(
                "🔴 [USER ERROR] :: Invalid user input at → shell_maid_av",
                "yellow",
            );
            return false;
        }
    }
}
