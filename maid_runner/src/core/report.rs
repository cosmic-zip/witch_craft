use crate::core::structs::{Logger, ProcessResult};
use crate::core::utils::*;
use crate::meow::meow::read_meow;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn json_filter(line: &str) -> String {
    let mut filtered_line = "".to_string();
    for symbol in line.chars() {
        if symbol == '"' {
            filtered_line = format!("{}{}", filtered_line, "\"");
        } else if symbol == '\\' {
            filtered_line = format!("{}{}", filtered_line, "");
        } else if symbol == '\n' {
            filtered_line = format!("{}{}", filtered_line, "");
        } else if symbol == '\t' {
            // tab to 4
            filtered_line = format!("{}{}", filtered_line, "    ");
        } else {
            filtered_line = format!("{}{}", filtered_line, symbol);
        }
    }
    return filtered_line;
}

pub fn format_std(data: String) -> Vec<String> {
    let mut formated: Vec<String> = Vec::new();
    let mut lines = data.lines();
    loop {
        match lines.next() {
            Some(line) => {
                let filtered_line = json_filter(line);
                formated.push(filtered_line);
            }
            None => break,
        }
    }
    return formated;
}

pub fn logger(data: Logger) -> std::io::Result<()> {

    let mut formated_stdout: Vec<String> = format_std(data.stdout);
    let mut formated_stderr: Vec<String> = format_std(data.stderr);

    let cmd: Vec<String> = data.source.split_whitespace().map(String::from).collect();

    let report_config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let report = format!(
        "{}{}",
        report_config["REPORT_BASE_PATH"], report_config["REPORT_LOG"]
    );

    let session_config = read_meow("/var/maid/maid_lists/embedded/session.meow", false);
    let session = session_config["LATEST_SESSION"].to_string();
    let session_description = session_config["DESCRIPTION"].to_string();

    let proccess_result = ProcessResult {
        session: session.as_str(),
        session_description: session_description.as_str(),
        source_from: data.source_from.as_str(),
        source_command: cmd[0].as_str(),
        source_detail: data.source.as_str(),
        source_description: data.source_description.as_str(),
        timestemp: system_time().as_str(),
        returned_status: data.status.as_str(),
        formated_stdout: formated_stdout,
        formated_stderr: formated_stderr,
        debug: data.debug,
    };

    // let file = File::options().append(true).open(&report);
    // let _ = file.expect("FILE").write_all(&proccess_result.as_bytes());
    Ok(())
}
