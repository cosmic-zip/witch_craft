use crate::modules::core::data::*;
use std::process::{Command, Output};
use std::env;

pub fn readargs() -> Vec<String> {
    return env::args().collect();
}

pub fn raise(arg: &str, fancy: i32) -> String {
    let fc = fancy as usize;
    let opts = vec![
        "⚪ [ message ]",
        "🟢 [ done ]",
        "🔴 [ fail ]", 
        "🟠 [ warning ]",
        "💀 [ error ]",
    ];

    if fc > opts.len(){
        return "🔴 Index overflow at @raise function".to_string()
    }

    return format!("\x1b[1m{}\x1b[0m :: {}", opts[fc].to_uppercase(), arg);
}

pub fn search_value(term: String, vector: Vec<String>) -> String {
    let mut counter = 0;

    while counter < vector.len() {
        if counter + 1 < vector.len() {
            if vector[counter].contains(SPLIT_I) {
                let keyname = vector[counter].replace(SPLIT_I, "");
                if keyname == term {
                    return vector[counter + 1].to_string();
                }
            }

            if vector[counter].contains(SPLIT_II) {
                let keyname = vector[counter].replace(SPLIT_II, "");
                if keyname == term {
                    return vector[counter + 1].to_string();
                }
            }
        }
        counter += 1;
    }

    println!("{}", raise(
        &format!("No value found for {} → Send empty string", term), 0));
    return "".to_string()
}

pub fn search_key(key: String, vector: Vec<String>) -> String {
    for item in vector {
        if item == key {
            return item;
        }
    }
    println!("{}",raise("Not found!", 0));
    return "none".to_string()
}

pub fn lazy_loop(meta_string: &str, argsv: Vec<String>) -> String {
    let meta: Vec<&str> = meta_string.split_whitespace().collect();
    let mut cmds: String = meta_string.to_string();

    for item in meta {
        if item.contains(TONK) {
            let opt = item.replace(TONK, "");
            let val = search_value(opt, argsv.clone());
            cmds = cmds.replace(item, &val);
        }
    }

    return cmds;
}

pub fn raw_exec(command_line: String) -> Option<Output> {
    let mut parts = command_line.split_whitespace();
    let command = parts.next().expect("No command found");
    let args: Vec<&str> = parts.collect();

    match Command::new(command).args(&args).output() {
        Ok(output) => Some(output),
        Err(_) => None,
    }
}

pub fn lazy_exec(command_line: String, pretty: bool) -> i32 {
    match raw_exec(command_line) {
        Some(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if pretty {
                    raise("Start", 4);
                }
                println!("\n{}\n", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if pretty {
                    raise("Error", 4);
                }
                eprintln!("\n{}\n", stderr);
            }
            output.status.code().unwrap_or(-1)
        }
        None => {
            return 0
        }
    }
}


