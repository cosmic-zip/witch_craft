use std::fs::OpenOptions;
use crate::datetime_now;
use std::io::Write;
use super::witchrc::readrc_value;
use serde::{Serialize, Deserialize};

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

        let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&witchrc)
                .unwrap();

        if let Err(e) = writeln!(file, "{}", output) {
            eprintln!("Couldn't write to file: {}", e);
        }

        output
    }

}
