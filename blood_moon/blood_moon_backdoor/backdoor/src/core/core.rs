use reqwest;
use crate::core::setup::URL;
use crate::core::structs::{CommandCall, CommandResult, ProcessInit};
use std::process::{Command, Output};
use std::io::Error;

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
        Err(_err) => {
            println!("Failed to upgrade, try to run as administrator.");
            return false;
        }
    }
}


pub fn command_finder() -> String {

    let response = reqwest::blocking::get(URL).expect("error");

    if response.status().is_success() {
        return response.text().expect("error");
    }

    "failed".to_string()
}