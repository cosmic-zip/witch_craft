use std::{fs::OpenOptions, process::Output};
use crate::datetime_now;
use std::io::Write;
use super::witchrc::readrc_value;
use serde::{Serialize, Deserialize};
use crate::core::core::get_os_env;

#[derive(Serialize, Deserialize)]
struct WitchyLogger {
    cmd_output: String,
    cmd_status: String,
    cmd_error: String,
    cmd_true: String,
    cmd_witchy: String,
    datetime_now: String,
}

impl WitchyLogger {
    fn new(
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

    fn empty() -> Self {
        WitchyLogger {
            cmd_output: String::new(),
            cmd_status: String::new(),
            cmd_error: String::new(),
            cmd_true: String::new(),
            cmd_witchy: String::new(),
            datetime_now: datetime_now(),
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn save(&self) -> String {

        let output = serde_json::to_string(self).unwrap();
        let witchrc = readrc_value("path_log_file");
        let home = get_os_env("HOME");
        let path = witchrc.replace("~/", &home);

        let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&path)
                .unwrap();

        if let Err(e) = writeln!(file, "{}", output) {
            eprintln!("Couldn't write to file: {}", e);
        }

        output
    }

}


pub fn core_logger(output: &Output, command_line: &String) -> bool {
    let logger = WitchyLogger::new(
        String::from_utf8_lossy(&output.stdout).to_string(),
        output.status.code().unwrap_or(156).to_string().to_owned().clone(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        command_line.to_string(),
        String::from("Some witchy details"),
    );

    if logger.save().is_empty() {
        return false;
    }

    println!("{}", logger.to_json());

    true

}
