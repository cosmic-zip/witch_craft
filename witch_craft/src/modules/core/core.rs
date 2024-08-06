use crate::modules::core::data::*;
use crate::modules::core::structs::DataSet;

use std::env;
use std::process::{Command, Output};

pub fn readargs() -> Vec<String> {
    return env::args().collect();
}

pub fn raise(arg: &str, fancy: i32) -> String {
    let fc = fancy as usize;
    let opts = vec![
        "🟣 [ message ]",
        "🟢 [ done ]",
        "🔴 [ fail ]",
        "🟠 [ warning ]",
        "💀 [ error ]",
        "🟣 [ manual ]",
    ];

    if fc >= opts.len() {
        return "🔴 Index overflow at @raise function".to_string();
    }

    let out = format!("\x1b[1m{}\x1b[0m :: {}", opts[fc].to_uppercase(), arg);

    println!("\n{}\n", out);
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

pub fn lazy_loop(meta_string: &str, argsv: Vec<String>) -> String {
    let meta: Vec<&str> = meta_string.split_whitespace().collect();
    let mut cmds: String = meta_string.to_string();

    for item in meta {
        if item.contains("http") {
            // Fix for urls like https://@@domain/some/thing
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

pub fn raw_exec(command_line: String) -> Option<Output> {
    let mut parts = command_line.split_whitespace();
    let command = parts.next().expect("No command found");
    let args: Vec<&str> = parts.collect();

    match Command::new(command).args(&args).output() {
        Ok(output) => Some(output),
        Err(_) => None,
    }
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

pub fn bob(set: DataSet, argsv: Vec<String>) -> i32 {
    raise(&set.name, 0);
    let cmd = lazy_loop(&set.meta, argsv);
    let out = lazy_exec(cmd, false);
    if out != 0 {
        return out;
    }

    return 0;
}

pub fn magic_docs() {
    let data = data();
    for dataset in data {
        if dataset.docs == "" {
            println!("\nNo manual page found");
        } else {
            raise(&dataset.docs, 5);
        }

        println!("\n\t{}", dataset.name);

        let options: Vec<&str> = dataset.meta.split(" ").collect();
        for opt in options {
            if opt.contains(TONK) {
                println!("\t{}{}", SPLIT_II, opt.replace(TONK, ""));
            }
        }
    }
}

pub fn check_install() -> bool {
    let packages: Vec<&str> = vec![
            "nmap",
            "dirb",
            "dnsenum",
            "libc-bin",
            "iproute2",
            "xxd",
            "iptables",
            "coreutils",
            "wget",
            "curl",
            "dnsutils",
            "traceroute",
            "openssl",
            "openssh-server",
            "xattr",
            "libimage-exiftool-perl",
            "tor",
            "foremost",
    ];

    for pkg in packages {
        let out = lazy_exec(pkg.to_string(), true);
        // code 127 are for not found
        if out == 127 {
            raise(&format!("Fail! {}", pkg), 4);
            return false;
        }
    }

    raise("All checks pass!", 1);
    return true;
}
