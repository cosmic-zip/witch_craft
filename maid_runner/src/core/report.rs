use crate::core::utils::*;
use crate::meow::meow::read_meow;
use std::fs::File;
use std::io::prelude::*;

pub fn json_filter(line: &str) -> String {
    let mut filtered_line = "".to_string();
    for symbol in line.chars() {
        if symbol == '"' {
            filtered_line = format!("{}{}", filtered_line, "\\\"");
        } else if symbol == '\\' {
            filtered_line = format!("{}{}", filtered_line, "\\\\");
        } else if symbol == '\t' {
            // tab to 4
            filtered_line = format!("{}{}", filtered_line, "    ");
        } else {
            filtered_line = format!("{}{}", filtered_line, symbol);
        }
    }
    return filtered_line;
}

pub fn write_report(
    command_string: String,
    status: String,
    stdout: String,
    stderr: String,
    _debug: bool,
) -> std::io::Result<()> {
    let mut lines = stdout.lines();
    let mut formated_stdout = "\"\"".to_string();
    loop {
        match lines.next() {
            Some(line) => {
                let filtered_line = json_filter(line);
                let span = format!("\"{}\"", filtered_line);
                formated_stdout = format!("{}, {}", formated_stdout, span);
            }
            None => break,
        }
    }

    let cmd: Vec<String> = command_string
        .split_whitespace()
        .map(String::from)
        .collect();

    let config = read_meow("/var/maid/files/embedded/config.meow", true);
    let report = format!("{}{}", config["REPORT_BASE_PATH"], config["REPORT_LOG"]);
    let session = format!("{}{}", config["REPORT_BASE_PATH"], config["SAFE_LOOK"]);

    let contents = format!(
        "{{
            \"session\": \"{}\", 
            \"command_base\": \"{}\", 
            \"timestemp\": \"{}\", 
            \"command_status\": {}, 
            \"command_error\": {}, 
            \"command_string\": \"{}\", 
            \"formated_stdout\": [ {} ] 
        }}\n",
        session,
        cmd[0],
        system_time(),
        status,
        format!("\"{}\"", stderr),
        command_string,
        formated_stdout,
    );

    let file = File::options().append(true).open(&report);
    let _ = file.expect("FILE").write_all(&contents.as_bytes());
    Ok(())
}
