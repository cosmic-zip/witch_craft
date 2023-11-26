use crate::core::structs::{Logger, ProcessResult};
use crate::core::messages::standard_messages;
use crate::core::utils::*;
use crate::meow::meow::read_meow;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

extern crate rusqlite;

use rusqlite::{params, Connection, Result};

pub fn write_process_result_to_db<'a>(
    conn: &Connection,
    result: ProcessResult<'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
) -> Result<()> {

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS process_results (
            id INTEGER PRIMARY KEY,
            session TEXT,
            session_description TEXT,
            source_from TEXT,
            source_command TEXT,
            source_detail TEXT,
            source_description TEXT,
            timestemp TEXT,
            returned_status TEXT,
            formated_stdout TEXT,
            formated_stderr TEXT,
            debug INTEGER
        )",
        [],
    );

    conn.execute(
        "INSERT INTO process_results (session, session_description, source_from, source_command, source_detail, source_description, timestemp, returned_status, formated_stdout, formated_stderr, debug)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            result.session,
            result.session_description,
            result.source_from,
            result.source_command,
            result.source_detail,
            result.source_description,
            result.timestemp,
            result.returned_status,
            result.formated_stdout.join("\n"),
            result.formated_stderr.join("\n"),
            result.debug,
        ],
    )?;
    Ok(())
}


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

    let binding = system_time();

    let proccess_result = ProcessResult {
        session: session.as_str(),
        session_description: session_description.as_str(),
        source_from: data.source_from.as_str(),
        source_command: cmd[0].as_str(),
        source_detail: data.source.as_str(),
        source_description: data.source_description.as_str(),
        timestemp: binding.as_str(),
        returned_status: data.status.as_str(),
        formated_stdout: formated_stdout,
        formated_stderr: formated_stderr,
        debug: data.debug,
    };

    let connection = Connection::open(report);
    match connection {
        Ok(conn) => {
            let _ = write_process_result_to_db(&conn, proccess_result);
        }
        Err(err) => {
            standard_messages("error", "Database connection failed", "report.logger", "cute");
        }
    } 

    Ok(())
}
