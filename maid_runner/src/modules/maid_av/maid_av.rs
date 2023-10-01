use crate::core::utils::*;
use crate::meow::meow::read_meow;

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
        "[MALWARE_PATTERN] :: Searching for malware pattern",
        "green",
    );

    match find_all_matching_lines(malware_db, pattern) {
        Ok(result) => {
            if !result.is_empty() {
                for line in result {
                    println!("🚧 {}", line);
                }
                true
            } else {
                println!("🔴 [WARNING] :: Pattern not found in any line.");
                false
            }
        }
        Err(err) => {
            eprintln!("🔴 [ERROR] :: {}", err);
            false
        }
    }
}

pub fn shell_maid_av(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--scanner" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = &gsv(system_input.clone(), "--hash");

            search_malware_hash(instance, debug)
        }

        "--pattern" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = &gsv(system_input.clone(), "--pattern");

            search_malware_pattern(instance, debug)
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
