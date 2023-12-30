use crate::core::messages::standard_messages;
use crate::core::report::*;
use crate::core::structs::{CommandCall, CommandResult, Logger, ProcessInit};
use colored::*;
use csv::ReaderBuilder;
use std::env;
use std::error::Error as StdError;
use std::error::Error as Err;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, Error, Read};
use std::mem;
use std::path::Path;
use std::process::{Command, Output};

use chrono;
use hex;

pub fn system_command_call(cmd: CommandCall) -> Result<CommandResult, Error> {
    let output: Output = Command::new(cmd.command).args(cmd.args).output()?;

    let status_code = output.status.code().unwrap_or(-1).to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let result = CommandResult {
        status: status_code,
        stdout: stdout,
        stderr: stderr,
    };

    Ok(result)
}

pub fn system_string_to_vec_builder(build_string: &str) -> Vec<String> {
    build_string
        .to_string()
        .split_whitespace()
        .map(String::from)
        .collect()
}

pub fn system_command_exec(command: ProcessInit) -> bool {
    let string_args = system_string_to_vec_builder(command.source);
    let command_call: CommandCall = CommandCall {
        command: string_args[0].as_str(),
        args: &string_args[1..],
    };

    // Dummy mockup
    if command.source == "DUMP_TBV" {
        return true;
    }

    match system_command_call(command_call) {
        Ok(res) => {
            let (status, stdout, stderr) = (res.status, res.stdout, res.stderr);

            let data = Logger {
                source: command.source.to_string(),
                source_from: command.source_from.to_string(),
                source_description: command.source_description.to_string(),
                status: status.to_string(),
                stdout: stdout.to_string(),
                stderr: stderr.to_string(),
                debug: command.debug,
            };

            match logger(data) {
                Ok(()) => {}
                Err(err) => {
                    standard_messages(
                        "error",
                        "While executing command logger",
                        &format!("{}", err),
                        "cute",
                    );
                    return false;
                }
            }

            if command.debug == true {
                println!(
                    "🔖 status: {}\n🚧 stdout:\n{}\n🚧 stderr:\n{}\n",
                    status, stdout, stderr,
                );
            } else {
                println!("{}", stdout);
            }

            return true;
        }
        Err(err) => {
            standard_messages(
                "error",
                "While executing command system_command_call",
                &format!("{}", err),
                "cute",
            );
            return false;
        }
    }
}

pub fn system_text(text: &str, color: &str) -> bool {
    let mut lines = text.lines();
    loop {
        match lines.next() {
            Some(line) => {
                if color == "black" {
                    println!("{}", line.black())
                } else if color == "red" {
                    println!("{}", line.red())
                } else if color == "green" {
                    println!("{}", line.green())
                } else if color == "yellow" {
                    println!("{}", line.yellow())
                } else if color == "blue" {
                    println!("{}", line.blue())
                } else if color == "purple" {
                    println!("{}", line.purple())
                } else if color == "cyan" {
                    println!("{}", line.cyan())
                } else {
                    println!("{}", line.white())
                }
            }
            None => break,
        }
    }
    println!("");
    return true;
}

pub fn system_time() -> String {
    format!("{:?}", chrono::offset::Local::now())
}

pub fn system_exec_shell(debug: bool) -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if debug == true {
        standard_messages(
            "error",
            "System execute sub process ",
            "system_exec_shell",
            "cute",
        );
        println!("{:?}", args);
    }
    return args;
}

pub fn take_system_args(mut data: &mut Vec<String>, parameter_name: &str) -> String {
    if data.len() <= 2 {
        return "[GSV] :: Invalid vector size at → gsv".to_string();
    } 

    let mut count = 2;
    while count < data.len() {
        if data[count] == parameter_name {
            return mem::take(&mut data[count + 1]);
        }
        count += 1;
    }

    let hardcoded_debug: bool = false;

    if hardcoded_debug == true {
        if parameter_name == "--debug" {
            // Drop error message if debug flag is missing
            return "".to_string();
        }
    
        let message = format!(
            "[GSV] :: Parameter name not found at → P: {}, Vec: {:?}",
            parameter_name, data
        );
    
        return standard_messages("error", &message, "utils::take_system_args", "cute");
    } else {
        return "".to_string();
    }

}

pub fn take_system_args_debug(debug: String) -> bool {
    if debug == "false" {
        return false;
    }

    return true;
}

pub fn search_csv(file_path: &str, search_term: &str) -> Result<Vec<String>, Box<dyn Err>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut matching_rows = Vec::new();

    for result in reader.records() {
        let record = result?;
        for field in record.iter() {
            if field.contains(search_term) {
                matching_rows.push(record.as_slice().to_string());
                break;
            }
        }
    }

    Ok(matching_rows)
}

pub fn find_all_matching_lines(file_path: &str, pattern: &str) -> Result<Vec<String>, Error> {
    let file = File::open(file_path)?;
    let mut matching_lines = Vec::new();

    for (line_num, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            matching_lines.push(format!("Line {}: {}", line_num + 1, line));
        }
    }

    let command = format!(
        "find_all_matching_lines pattern: {}, file_path : {}",
        pattern, file_path
    );

    let data = Logger {
        source: command,
        source_from: "utils".to_string(),
        source_description: "Finding all matching lines on a file".to_string(),
        status: 0.to_string(),
        stdout: format!("{:?}", matching_lines),
        stderr: "none".to_string(),
        debug: true,
    };

    match logger(data) {
        Ok(_result) => {
            // standard_messages("saved", "Log saved", "", "cute");
        }
        Err(_err) => println!("Error"),
    }

    Ok(matching_lines)
}

pub fn read_file_to_string(file_path: &str, _debug: bool) -> String {
    match File::open(file_path) {
        Ok(file) => {
            use std::io::Read;
            let mut reader = std::io::BufReader::new(file);
            let mut content = String::new();
            reader
                .read_to_string(&mut content)
                .expect("🚧 [ERROR] Unable to read the File");
            content
        }
        Err(_) => String::from("🚧 [ERROR] Unable to read the File"),
    }
}

pub fn read_file_to_hex(file_path: &str, debug: bool) -> Vec<String> {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .expect("Failed to read the file");
            let hex_encoded = hex::encode(&buffer);
            let mut hex = Vec::new();
            let mut count = "".to_string();
            for hex_code in hex_encoded.chars() {
                if count.len() == 4 {
                    hex.push(count);
                    count = "".to_string();
                }

                count = format!("{}{}", count, hex_code);
            }

            if debug == true {
                println!("{:?}", hex);
            }

            return hex;
        }
        Err(_) => {
            standard_messages(
                "error",
                "Unable to read the file",
                "read_file_to_hex",
                "cute",
            );
            return vec![];
        }
    }
}
