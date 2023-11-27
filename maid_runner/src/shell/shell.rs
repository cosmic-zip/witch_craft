use crate::core::{core::*, manual::*, messages::*, structs::*, utils::*};

use crate::modules::{
    curl::curl::*,
    firewall::firewall::*,
    // botnet::botnet::*,
    lookup::lookup::*,
    // attack::attack::*,
    // iso::iso::*,
    maid_av::maid_av::*,
    // rootkit::rootkit::*,
    scanner::scanner::*,
};

// command structure
// bin [option] [value] ... [option_N] [value_N]

pub fn init() -> u8 {
    let mut system_input: &mut Vec<String> = &mut system_exec_shell(false);

    if system_input.len() < 3 {
        system_exec_manual("default");
        return 1;
    }

    let module_name = system_input[1].as_str();

    match module_name {
        "core" | "c" => {
            shell_core(system_input);
        }

        "maid_av" | "av" => {
            shell_maid_av(system_input);
        }

        "lookup" | "l" => {
            shell_lookup(system_input);
        }

        "scanner" | "s" => {
            shell_scanner(system_input);
        }

        "bcurl" => {
            shell_curl(system_input);
        }

        "help" | "h" => {
            shell_manual(system_input);
        }

        "firewall" | "f" => {
            shell_firewall(system_input);
        }

        _ => {
            let mut cmd = "".to_string();
            for item in &system_input[1..] {
                cmd = format!("{} {}", cmd, item);
            }

            let instance = ProcessInit {
                source: &cmd,
                source_from: "shell",
                source_description: "Trying execute the given external command",
                debug: true,
            };
            system_command_exec(instance);
        }
    }

    return 1;
}
