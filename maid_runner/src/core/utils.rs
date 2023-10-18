use crate::core::messages::standard_messages;
use crate::core::report::*;
use crate::core::structs::{CommandCall, CommandResult};
use colored::*;
use csv::ReaderBuilder;

use std::env;
use std::error::Error as StdError;
use std::error::Error as Err;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, Error, Read};
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

pub fn system_string_to_vec_builder(build_string: String) -> Result<Vec<String>, Error> {
    let sys_args: Vec<String> = build_string.split_whitespace().map(String::from).collect();
    Ok(sys_args)
}

pub fn system_command_exec(source: &str, source_description: &str, debug: bool) -> bool {
    let sourceing = source.to_string();

    match system_string_to_vec_builder(sourceing.clone()) {
        Ok(res_command_call) => {
            let command_call: CommandCall = CommandCall {
                command: res_command_call[0].as_str(),
                args: &res_command_call[1..],
            };

            match system_command_call(command_call) {
                Ok(res) => {
                    if debug == true {
                        let (x, y, z) = (res.status, res.stdout, res.stderr);
                        println!("🔖 status: {} \n🚧 STDOUT: {}\n🚧 STDERR: {}\n\n", x, y, z);

                        match write_report(sourceing.clone(), source_description.to_string(), x, y, z, debug) {
                            Ok(()) => {
                                standard_messages("saved", "Report created", "", "cute");
                            }

                            Err(err) => {
                                let message = format!("{}", err);
                                standard_messages(
                                    "error",
                                    "While executing command write_report",
                                    &message,
                                    "cute",
                                );
                                return false;
                            }
                        }

                        return true;
                    }

                    // Print sub process output
                    println!("\n{}\n\n", res.stdout);
                    if res.stderr != "" {
                        println!("\n{}\n\n", res.stderr);
                    }
                    return true;
                }
                Err(err) => {
                    let message = format!("{}", err);
                    standard_messages(
                        "error",
                        "While executing command system_command_call",
                        &message,
                        "cute",
                    );
                    return false;
                }
            }
        }
        Err(err) => {
            let message = format!("{}", err);
            standard_messages(
                "error",
                "While executing command system_string_to_vec_builder",
                &message,
                "cute",
            );
            return false;
        }
    }
}

pub fn system_command_deep_exec(source: &str, _debug: bool) -> Result<CommandResult, Error> {
    let sourceing = source.to_string();
    let mut result = CommandResult {
        status: "none".to_string(),
        stdout: "none".to_string(),
        stderr: "none".to_string(),
    };

    match system_string_to_vec_builder(sourceing) {
        Ok(res_command_call) => {
            let command_call: CommandCall = CommandCall {
                command: res_command_call[0].as_str(),
                args: &res_command_call[1..],
            };

            match system_command_call(command_call) {
                Ok(res) => result = res,
                Err(err) => {
                    let message = format!("{}", err);
                    standard_messages(
                        "error",
                        "While executing command system_string_to_vec_builder",
                        &message,
                        "cute",
                    );
                }
            }
        }
        Err(err) => {
            let message = format!("{}", err);
            standard_messages(
                "error",
                "While executing command system_string_to_vec_builder",
                &message,
                "cute",
            );
        }
    }

    Ok(result)
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

pub fn gsv(data: Vec<String>, parameter_name: &str) -> String {
    if data.len() <= 2 {
        return "🚧 [GSV] :: Invalid vector size at → gsv".to_string();
    }

    let mut count = 2;
    while count < data.len() {
        if data[count] == parameter_name {
            return data[count + 1].to_string();
        }
        count += 1;
    }

    return format!(
        "🚧 [GSV] :: Parameter name not found at → P: {}, Vec: {:?}",
        parameter_name, data
    );
}

pub fn gsv_debug(debug: String) -> bool {
    if debug == "false" {
        return false;
    } else {
        return true;
    }
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
                // println!("@{:?}", matching_rows[0]);
                break;
            }
        }
    }

    Ok(matching_rows)
}

pub fn find_all_matching_lines(
    file_path: &str,
    pattern: &str,
) -> Result<Vec<String>, Box<dyn StdError>> {
    let file = File::open(file_path)?;
    let mut matching_lines = Vec::new();

    for (line_num, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            matching_lines.push(format!("Line {}: {}", line_num + 1, line));
        }
    }

    match write_report(
        format!(
            "find_all_matching_lines pattern: {}, file_path : {}",
            pattern, file_path
        ),
        "Find matching lines inside an file".to_string(),
        "0".to_string(),
        format!("{:?}", matching_lines),
        "None".to_string(),
        false,
    ) {
        Ok(()) => {
            // system_text("[REPORT] :: Report created", "green");
        }

        Err(err) => {
            let message = format!("{}", err);
            standard_messages(
                "error",
                "While executing command system_command_call",
                &message,
                "cute",
            );
        }
    }

    Ok(matching_lines)
}

pub fn read_file_to_string(file_path: &str, debug: bool) -> String {
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
