use crate::core::consts::*;
use crate::core::data::*;
use crate::core::structs::DataSet;
use chrono;
use colored::*;
use regex::Regex;
use std::env;
use std::fs;
use std::net::IpAddr;
use std::path::Path;
use std::process::{Command, Output};
use std::str::FromStr;

use super::types::Closure;

pub fn readargs() -> Vec<String> {
    env::args().collect()
}

pub fn datetime_now() -> String {
    let time = chrono::offset::Local::now();
    return time.to_string();
}


/// Raises a formatted message based on the provided warning type.
///
/// # Arguments
///
/// * `arg` - A string slice that holds the message to be displayed.
/// * `warning_type` - A string slice that specifies the type of warning.
///   Valid options are:
///   - "message": Indicates a general message.
///   - "done": Indicates a successful operation.
///   - "fail": Indicates a failure.
///   - "warning": Indicates a warning condition.
///   - "error": Indicates an error.
///   - "entry": Indicates an entry point message.
///
/// # Returns
///
/// A formatted string that combines the selected warning type with the provided message.
/// If an invalid `warning_type` is given, an error message will be returned.
///
/// # Example
///
/// ```rust
/// let result = raise("Operation completed", "done");
/// println!("{}", result); // Output: "🟢 [ DONE ] :: Operation completed"
/// ```
pub fn raise(arg: &str, warning_type: &str) -> String {
    let opts = [
        "🟣 [ system says ] ::",
        "🟢 [ well done ] ::",
        "🔴 [ fail ] ::",
        "🟠 [ warning ] ::",
        "💀 [ burh ] ::",
        "🔘 [ entry point ] ::",
    ];

    // Match the warning_type to find the corresponding option
    let out = match warning_type {
        "message" => opts[0],
        "done" => opts[1],
        "fail" => opts[2],
        "warning" => opts[3],
        "error" => opts[4],
        "entry" => opts[5],
        _ => "" // Empty message
    };

    let formatted_output = format!("{} {}", out.to_uppercase(), arg);

    println!("\n{}\n", formatted_output.bold());
    formatted_output
}


pub fn search_value(key: &str, vector: &[String]) -> String {
    let mut counter = 0;

    while counter < vector.len() {
        if counter + 1 < vector.len() {
            if vector[counter].contains(SPLIT_I) {
                let key_name = vector[counter].replace(SPLIT_I, "");
                if key_name == key {
                    return vector[counter + 1].to_string();
                }
            }

            if vector[counter].contains(SPLIT_II) {
                let key_name = vector[counter].replace(SPLIT_II, "");
                if key_name == key {
                    return vector[counter + 1].to_string();
                }
            }
        }
        counter += 1;
    }

    println!(
        "{}",
        raise(
            &format!("No value found for {} → Send empty string", key),
            "warning"
        )
    );
    String::new()
}

pub fn search_key(key: &str, vector: &[String]) -> String {
    for item in vector {
        if item == key {
            return item.to_string();
        }
    }
    println!("{}", raise(&format!("Key not found! {}", key), "warning"));
    String::new()
}

#[allow(dead_code)]
pub fn seach_number_value(key: &str, argsv: &[String]) -> i32 {
    search_value(key, argsv).parse::<i32>().unwrap_or(0)
}

fn witch_fmt(input: &str, max_length: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut start = 0;

    while start < input.len() {
        let mut end = std::cmp::min(start + max_length, input.len());
        if end < input.len() && !input[end..end + 1].chars().all(char::is_whitespace) {
            if let Some(space_index) = input[..end].rfind(|c: char| c.is_whitespace()) {
                end = space_index + 1;
            }
        }
        if end == start {
            end = std::cmp::min(start + max_length, input.len());
        }
        lines.push(input[start..end].to_string());
        start = end;
    }
    lines
}

pub fn magic_docs() -> i32 {
    let data: Vec<DataSet> = data();

    if data.is_empty() {
        raise("Datasets is empty", "fail");
        return 42;
    }

    println!("{}", PANZER_MAID);
    raise(MAN_HEADER, "");

    fn loop_parser(arg_name: &str) -> Vec<String> {
        for tuple in MAGIC_DOCS {
            if tuple.0 == arg_name.replace("--", "") {
                return witch_fmt(
                    &format!("{}--{} :: {}", " ".repeat(8), tuple.0, tuple.1),
                    72,
                );
            }
        }
        witch_fmt(&format!("{}{}", " ".repeat(8), arg_name), 72)
    }

    for dataset in data {
        let header =
            witch_fmt(&format!("    {} ► {}", dataset.name, dataset.docs), 72).join("\n     ");
        raise(&header, "");

        let mut out: String = dataset.meta.to_string();
        out = out.replace("/", "");
        out = out.replace(",", "");
        out = out.replace("'", "");
        out = out.replace("\"", "");
        out = out.replace(";", "");
        out = out.replace(":", " ");
        out = out.replace("@@@", "@ @@");
        let args: Vec<_> = out.split(" ").collect();
        for arg in args {
            if arg.contains("@@") {
                let out = arg.replace(TONK, "--");
                let re = Regex::new(r"^.*?--").unwrap();
                let result = re.replace_all(&out, "--").to_string();

                let lp = loop_parser(&result);
                for item in lp {
                    println!("{}", item);
                }
            }
        }
    }

    return 0;
}

pub fn lazy_parser(meta_string: &str, argsv: &[String]) -> String {
    let meta: Vec<&str> = meta_string.split_whitespace().collect();
    let mut cmds: String = meta_string.to_string();

    for item in meta {
        if item.contains("http") {
            let aaaa = item.split("/");
            let mut new = String::new();

            for c in aaaa {
                if c.contains(TONK) {
                    let opt = c.replace(TONK, "");
                    let val = search_value(&opt, argsv);
                    new = item.replace(c, &val);
                }
            }

            cmds = cmds.replace(item, &new);
        }

        if item.contains(TONK) & !item.contains("http") {
            let opt = item.replace(TONK, "");
            let val = search_value(&opt, argsv);
            cmds = cmds.replace(item, &val);
        }
    }

    cmds
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

pub fn lazy_exec(command_line: String) -> i32 {
    match raw_exec(command_line) {
        Some(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines = stdout.split("\n");
                for line in lines {
                    let result = witch_fmt(line, 180);
                    for line in result {
                        println!("\t{}", line);
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("\n{}\n", stderr);
            }
            println!(" ");
            output.status.code().unwrap_or(-1)
        }
        None => 0,
    }
}

pub fn flawless_exec(set: DataSet, argsv: &[String]) -> i32 {
    raise(&set.name, "");
    let cmd = lazy_parser(&set.meta, argsv);
    lazy_exec(cmd)
}

pub fn directory_lookup(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(directory_lookup(&path));
        } else {
            files.push(path.to_string_lossy().to_string());
        }
    }

    files
}

pub fn closure_shell(options: Closure, argsv: &[String]) -> i32 {
    if argsv.len() % 2 != 0 {
        println!("{}", WITCH);
        return 0;
    }

    let name = argsv[1].as_str();

    for function in options {
        if function.0 == name {
            return function.1(argsv);
        }
    }

    11223300
}

pub fn read_file_to_lines(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(value) => value.lines().map(String::from).collect(),
        Err(err) => {
            raise(&format!("Error at {}", err), "message");
            return Vec::new();
        }
    }
}

pub fn ip_to_number(ip_str: &str) -> String {
    fn convert_addr(ip: IpAddr) -> u128 {
        match ip {
            IpAddr::V4(ipv4) => u128::from(u32::from_be_bytes(ipv4.octets())),
            IpAddr::V6(ipv6) => {
                let bytes = ipv6.octets();
                let mut num = 0u128;
                for byte in bytes {
                    num = (num << 8) | u128::from(byte);
                }
                num
            }
        }
    }

    match IpAddr::from_str(ip_str) {
        Ok(ip) => {
            let number = convert_addr(ip);
            number.to_string()
        }
        Err(e) => {
            println!("Failed to parse IP address '{}': {}", ip_str, e);
            String::new()
        }
    }
}
