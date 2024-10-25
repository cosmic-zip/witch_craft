use super::witchrc::witchy_readrc_value;
use crate::core::core::*;
use crate::core::witchrc::rc_exists;
use crate::datetime_now;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs::OpenOptions, process::Output};

#[derive(Serialize, Deserialize)]
pub struct WitchyLogger {
    cmd_output: String,
    cmd_status: String,
    cmd_error: String,
    cmd_true: String,
    cmd_witchy: String,
    datetime_now: String,
}

impl WitchyLogger {
    pub fn new(
        cmd_output: String,
        cmd_status: String,
        cmd_error: String,
        cmd_true: String,
        cmd_witchy: String,
    ) -> Self {
        WitchyLogger {
            cmd_output,
            cmd_status,
            cmd_error,
            cmd_true,
            cmd_witchy,
            datetime_now: datetime_now(),
        }
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        WitchyLogger {
            cmd_output: String::new(),
            cmd_status: String::new(),
            cmd_error: String::new(),
            cmd_true: String::new(),
            cmd_witchy: String::new(),
            datetime_now: datetime_now(),
        }
    }

    #[allow(dead_code)]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn save(&self) -> String {
        let output = serde_json::to_string(self).unwrap();
        let witchrc = witchy_readrc_value("path_log_file");
        let home = get_os_env("HOME");

        if witchrc.is_empty() || home.is_empty() {
            return String::new();
        }

        let path = witchrc.replace("~/", &home);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path);

        match file {
            Ok(mut file) => writeln!(file, "{}", output).unwrap(),
            Err(err) => {
                eprintln!("Couldn't write to file: {}", err);
            }
        };

        output
    }
}

/// A wrapper function that logs command execution details using `WitchyLogger`.
///
/// This function takes an `Output` object from a command execution and the command line string
/// that was executed. It uses the `WitchyLogger` struct to log the output, including standard
/// output, standard error, and the exit status of the command. This is primarily used in
/// conjunction with the `lazy_exec` function.
///
/// # Arguments
/// * `output` - An `Output` object containing the result of the command execution, including
///   `stdout`, `stderr`, and the exit status.
/// * `command_line` - A `&String` representing the command line string that was executed.
///
/// # Returns
/// A `bool` indicating whether the logging operation was successful (`true`) or not (`false`).
///
/// # Example
/// ```
/// let command_output = Output {
///     stdout: b"Command executed successfully".to_vec(),
///     stderr: b"".to_vec(),
///     status: ExitStatus::from_raw(0)
/// };
/// let command_line = "example.command --arg".to_string();
///
/// if core_logger(&command_output, &command_line) {
///     println!("Logging successful.");
/// } else {
///     println!("Logging failed.");
/// }
/// ```
///
/// # Note
/// Ensure that the `WitchyLogger` struct is properly initialized before calling this function.
/// The logging functionality is intended to capture and store the output of command executions
/// for debugging and record-keeping purposes.
pub fn core_logger(output: &Output, command_line: &String) -> bool {
    if rc_exists() == false {
        return true;
    }

    let logger = WitchyLogger::new(
        String::from_utf8_lossy(&output.stdout).to_string(),
        output
            .status
            .code()
            .unwrap_or(156)
            .to_string()
            .to_owned()
            .clone(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        command_line.to_string(),
        String::from("Some witchy details"),
    );

    if logger.save().is_empty() {
        return false;
    }

    return true;
}
