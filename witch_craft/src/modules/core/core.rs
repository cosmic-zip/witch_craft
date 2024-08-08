use crate::modules::core::data::*;
use crate::modules::core::structs::DataSet;
use crate::modules::core::consts::*;
use regex::Regex;
use colored::*;

use std::env;
use std::process::{Command, Output};

pub fn readargs() -> Vec<String> {
    return env::args().collect();
}

pub fn raise(arg: &str, fancy: i32) -> String {
    let fc = fancy as usize;
    let opts = vec![
        "🟣 [ message ] ::",
        "🟢 [ done ] ::",
        "🔴 [ fail ] ::",
        "🟠 [ warning ] ::",
        "💀 [ error ] ::",
        "🔘 [ entry ] ::",
        "", //6
    ];

    if fc >= opts.len() {
        return "Index overflow at @raise function".to_string();
    }

    let out = format!("{} {}", opts[fc].to_uppercase(), arg);

    println!("\n{}\n", out.bold());
    return out;
}

pub fn search_value(term: String, vector: Vec<String>) -> String {
    let mut counter = 0;

    while counter < vector.len() {
        if counter + 1 < vector.len() {
            if vector[counter].contains(SPLIT_I) {
                let key_name = vector[counter].replace(SPLIT_I, "");
                if key_name == term {
                    return vector[counter + 1].to_string();
                }
            }

            if vector[counter].contains(SPLIT_II) {
                let key_name = vector[counter].replace(SPLIT_II, "");
                if key_name == term {
                    return vector[counter + 1].to_string();
                }
            }
        }
        counter += 1;
    }

    println!(
        "{}",
        raise(
            &format!("No value found for {} → Send empty string", term),
            3
        )
    );
    return "".to_string();
}

pub fn search_key(key: String, vector: Vec<String>) -> String {
    for item in vector {
        if item == key {
            return item;
        }
    }
    println!("{}", raise("Not found!", 3));
    return "none".to_string();
}

fn split_line(input: &str, max_length: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut start = 0;
    while start < input.len() {
        let end = std::cmp::min(start + max_length, input.len());
        lines.push(input[start..end].to_string());
        start = end;
    }
    lines
}

/// Generates a pretty-formatted manual from `db.json`.
///
/// Parses command strings and adds fancy shell images in Chafa format.
/// Prints the formatted manual result.
///
/// # Functionality
/// - Parses command strings stored in `db.json`.
/// - Integrates Chafa images for visual enhancements.
/// - Outputs the result with improved formatting.
pub fn magic_docs() {
    let data: Vec<DataSet> = data();

    if data.is_empty() {
        raise("Datasets is empty", 1);
    }

    println!("{}", PANZER_MAID);

    fn loop_parser(arg_name: &str) -> String {
        for tuple in MAGIC_DOCS {
            if tuple.0 == arg_name.replace("--", "") {
                return format!("\t--{} :: {}", tuple.0, tuple.1);
            }
        }
        return format!("\t{}", arg_name);
    }

    for dataset in data {
        raise(&dataset.name, 5);

        let docs = format!("\t{}", &dataset.docs);
        raise(&docs, 6);

        let args: Vec<_> = dataset.meta.split(" ").collect();
        for arg in args {

            if arg.contains("@@") {

                let mut out: String = arg.replace("@@", "--");
                out = out.replace(":", "\n\t");
                out = out.replace(",", "");
                let re = Regex::new(r"^.*?--").unwrap();
                let result = re.replace_all(&out, "--").to_string();
                println!("{}", loop_parser(&result));

            }
        }

    }

}

/// Parses a UwU formatted command string.
///
/// Replaces placeholders `@@bar` in `meta_string` with values from `argsv`.
///
/// # Parameters
/// - `meta_string`: The command string to be parsed.
/// - `argsv`: A vector of strings where each pair represents a `--for` key and its value.
///
/// # Exemple:
/// - cmd: foo --flag @@bar
/// - input: foo --bar "some value"
/// - out: foo --flag "some value"
pub fn lazy_loop(meta_string: &str, argsv: Vec<String>) -> String {
    let meta: Vec<&str> = meta_string.split_whitespace().collect();
    let mut cmds: String = meta_string.to_string();

    for item in meta {
        if item.contains("http") {
            let aaaa = item.split("/");
            let mut new = String::new();

            for c in aaaa {
                if c.contains(TONK) {
                    let opt = c.replace(TONK, "");
                    let val = search_value(opt, argsv.clone());
                    new = item.replace(c, &val);
                }
            }

            cmds = cmds.replace(item, &new);
        }

        if item.contains(TONK) & !item.contains("http") {
            let opt = item.replace(TONK, "");
            let val = search_value(opt, argsv.clone());
            cmds = cmds.replace(item, &val);
        }
    }

    return cmds;
}

/// Executes a command string on the host system.
///
/// Takes a command string that has been parsed by `lazy_loop` or is a static command string.
/// Executes the command using the system's command interpreter and returns the output.
///
/// # Parameters
/// - `command_line`: The command string to execute.
///
/// # Returns
/// - `Option<Output>`: The output of the executed command. Returns `None` if the command execution fails.
pub fn raw_exec(command_line: String) -> Option<Output> {
    let mut parts = command_line.split_whitespace();
    let command = parts.next().expect("No command found");
    let args: Vec<&str> = parts.collect();

    match Command::new(command).args(&args).output() {
        Ok(output) => Some(output),
        Err(_) => None,
    }
}

/// A wrapper for `raw_exec` that handles command execution and output formatting.
///
/// Executes `raw_exec` with the given command string. If `pretty` is `true`, it formats the output.
/// Prints the command's standard output on success and the standard error if any errors occur.
/// Returns the exit status code of the command as an `i32`.
///
/// # Parameters
/// - `command_line`: The command string to execute.
/// - `pretty`: If `true`, formats the command output for better readability.
///
/// # Returns
/// - `i32`: The exit status code of the executed command, which is zero or greater if successful.
pub fn lazy_exec(command_line: String, pretty: bool) -> i32 {
    match raw_exec(command_line) {
        Some(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines = stdout.split("\n");
                for line in lines {
                    let result = split_line(&line, 180);
                    for line in result {
                        println!("\t{}", line);
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("\n{}\n", stderr);
            }
            println!("");
            output.status.code().unwrap_or(-1)
        }
        None => return 0,
    }
}

/// Executes the specified command (`cmd`) `count` times.
///
/// For example, `foo --count 10 --foo bar` runs `--foo bar` 10 times.
/// Useful for automation and stress testing.
///
/// # Notes
/// - Commands are executed immediately in interactive mode; no scheduling support.
///
/// # Arguments
/// - `cmd`: The command to execute.
/// - `count`: Number of executions.
///
/// # Example
/// ```
/// foo --count 5 --bar example
/// ```
pub fn lazy_exec_loop(argsv: Vec<String>, cmd: &str) -> i32 {
    let out = search_value("count".to_string(), argsv.clone());
    let range: i32 = out.parse().unwrap_or(1);
    let mut exit = 0;
    for i in 1..range {
        let exec = DataSet::from_str("name", "some.thing", cmd);
        exit = bob(exec, argsv.clone());
    }
    return exit;
}

/// Calls `lazy_exec` and `lazy_loop` with the provided arguments.
///
/// This function uses `DataSet` and `argsv` (a `Vec<String>` of terminal arguments) to:
/// - Parse and execute the command string found in `set.meta`.
///
/// # Arguments
/// - `set`: Contains the command metadata.
/// - `argsv`: Vector of terminal arguments to be parsed.
///
/// # Example
/// ```
/// let dataset = DataSet { /* ... */ };
/// let args = vec!["--flag".to_string(), "value".to_string()];
/// bob(dataset, args);
/// ```
pub fn bob(set: DataSet, argsv: Vec<String>) -> i32 {
    raise(&set.name, 0);
    let cmd = lazy_loop(&set.meta, argsv);
    return lazy_exec(cmd, false);
}
