use crate::modules::core::consts::*;
use crate::modules::core::data::*;
use crate::modules::core::structs::DataSet;
use colored::*;
use regex::Regex;
use std::env;
use std::process::{Command, Output};

pub fn readargs() -> Vec<String> {
    env::args().collect()
}

pub fn raise(arg: &str, fancy: i32) -> String {
    let fc = fancy as usize;
    let opts = [
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
    out
}

pub fn search_value(key: &str, vector: &[String]) -> String {
    let mut counter = 0;

    while counter < vector.len() {
        if counter + 1 < vector.len() {
            if vector[counter].contains(SPLIT_I) {
                let key_name = vector[counter].replace(SPLIT_I, "");
                if key_name == key {
                    return vector[counter + 1].to_string();
                }
            }

            if vector[counter].contains(SPLIT_II) {
                let key_name = vector[counter].replace(SPLIT_II, "");
                if key_name == key {
                    return vector[counter + 1].to_string();
                }
            }
        }
        counter += 1;
    }

    println!(
        "{}",
        raise(
            &format!("No value found for {} → Send empty string", key),
            3
        )
    );
    String::new()
}

pub fn search_key(key: &str, vector: &[String]) -> String {
    for item in vector {
        if item == key {
            return item.to_string();
        }
    }
    println!("{}", raise("Not found!", 3));
    String::new()
}

#[allow(dead_code)]
pub fn seach_number_value(key: &str, argsv: &[String]) -> i32 {
    search_value(key, argsv).parse::<i32>().unwrap_or(0)
}

/// Formats a string into multiple lines with a specified maximum length, similar to `fmt` in GNU utilities.
///
/// The `witch_fmt` function splits the input string into one or more lines, each of which does not exceed
/// the specified `max_length`. It attempts to avoid breaking words between lines. If a word is longer than
/// the maximum line length, it will be placed on its own line, potentially exceeding `max_length`.
///
/// # Parameters
///
/// - `input`: A string slice (`&str`) that contains the text to be formatted.
/// - `max_length`: The maximum number of characters allowed in each line. Lines will be wrapped to
///   adhere to this limit, but words will be preserved whole whenever possible.
///
/// # Returns
///
/// A `Vec<String>` where each `String` represents a line of text. Each line will be formatted to
/// have a length of up to `max_length`, with words kept intact as much as possible.
///
fn witch_fmt(input: &str, max_length: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut start = 0;

    while start < input.len() {
        let mut end = std::cmp::min(start + max_length, input.len());
        if end < input.len() && !input[end..end + 1].chars().all(char::is_whitespace) {
            if let Some(space_index) = input[..end].rfind(|c: char| c.is_whitespace()) {
                end = space_index + 1;
            }
        }
        if end == start {
            end = std::cmp::min(start + max_length, input.len());
        }
        lines.push(input[start..end].to_string());
        start = end;
    }
    lines
}

/// Generates a well-formatted manual from the `db.json` file.
///
/// This function parses command strings from the `db.json` file and incorporates
/// Chafa images for visual enhancements, producing a visually appealing and
/// readable manual. The final result is printed to the console with improved
/// formatting for better readability and presentation.
///
/// # Functionality
/// - **Parsing**: Reads and parses command strings stored in the `db.json` file.
/// - **Visual Enhancement**: Integrates Chafa images to enhance the visual appearance
///   of the output. Chafa is a tool that converts images into ASCII art for terminal display.
/// - **Formatting**: Outputs the manual with improved formatting to ensure the result
///   is both attractive and easy to read.
///
pub fn magic_docs() {
    let data: Vec<DataSet> = data();

    if data.is_empty() {
        raise("Datasets is empty", 1);
        return;
    }

    println!("{}", PANZER_MAID);
    raise(MAN_HEADER, 6);

    fn loop_parser(arg_name: &str) -> Vec<String> {
        for tuple in MAGIC_DOCS {
            if tuple.0 == arg_name.replace("--", "") {
                return witch_fmt(
                    &format!("{}--{} :: {}", " ".repeat(8), tuple.0, tuple.1),
                    72,
                );
            }
        }
        witch_fmt(&format!("{}{}", " ".repeat(8), arg_name), 72)
    }

    for dataset in data {
        let header =
            witch_fmt(&format!("    {} ► {}", dataset.name, dataset.docs), 72).join("\n     ");
        raise(&header, 6);

        let mut out: String = dataset.meta.to_string();
        out = out.replace("/", "");
        out = out.replace(",", "");
        out = out.replace("'", "");
        out = out.replace("\"", "");
        out = out.replace(";", "");
        out = out.replace(":", " ");
        out = out.replace("@@@", "@ @@");
        let args: Vec<_> = out.split(" ").collect();
        for arg in args {
            if arg.contains("@@") {
                let out = arg.replace(TONK, "--");
                let re = Regex::new(r"^.*?--").unwrap();
                let result = re.replace_all(&out, "--").to_string();

                let lp = loop_parser(&result);
                for item in lp {
                    println!("{}", item);
                }
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
pub fn lazy_parser(meta_string: &str, argsv: &[String]) -> String {
    let meta: Vec<&str> = meta_string.split_whitespace().collect();
    let mut cmds: String = meta_string.to_string();

    for item in meta {
        if item.contains("http") {
            let aaaa = item.split("/");
            let mut new = String::new();

            for c in aaaa {
                if c.contains(TONK) {
                    let opt = c.replace(TONK, "");
                    let val = search_value(&opt, &argsv);
                    new = item.replace(c, &val);
                }
            }

            cmds = cmds.replace(item, &new);
        }

        if item.contains(TONK) & !item.contains("http") {
            let opt = item.replace(TONK, "");
            let val = search_value(&opt, &argsv);
            cmds = cmds.replace(item, &val);
        }
    }

    cmds
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
///
/// # Returns
/// - `i32`: The exit status code of the executed command, which is zero or greater if successful.
pub fn lazy_exec(command_line: String) -> i32 {
    match raw_exec(command_line) {
        Some(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines = stdout.split("\n");
                for line in lines {
                    let result = witch_fmt(line, 180);
                    for line in result {
                        println!("\t{}", line);
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("\n{}\n", stderr);
            }
            println!(" ");
            output.status.code().unwrap_or(-1)
        }
        None => 0,
    }
}

/// Calls `lazy_exec` and `lazy_loop` with the provided arguments.
///
/// This function uses `DataSet` and `argsv` (a `&[String]` of terminal arguments) to:
/// - Parse and execute the command string found in `set.meta`.
///
/// # Arguments
/// - `set`: Contains the command metadata.
/// - `&argsv`: Vector of terminal arguments to be parsed.
///
/// # Example
/// ```
/// let dataset = DataSet { /* ... */ };
/// let args = vec!["--flag".to_string(), "value".to_string()];
/// flawless_exec(dataset, &args);
/// ```
pub fn flawless_exec(set: DataSet, argsv: &[String]) -> i32 {
    raise(&set.name, 0);
    let cmd = lazy_parser(&set.meta, argsv);
    lazy_exec(cmd)
}
