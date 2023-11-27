use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::firewall::firewall_structs::SimpleRule;
use std::mem;

pub fn firewall_preset(option: &str, debug: bool) -> bool {
    let mut rules = Vec::new();
    let mut result: bool = false;

    match option {
        "reset" => {
            rules = vec!["iptables -F"];
        }

        "kill" => {
            rules = vec![
                "iptables -F",
                "iptables -P INPUT DROP",
                "iptables -P FORWARD DROP",
                "iptables -P OUTPUT DROP",
            ];
        }

        "hardned" => {
            rules = vec![
                "iptables -F",
                "iptables -P INPUT DROP",
                "iptables -P FORWARD DROP",
                "iptables -P OUTPUT DROP",
                "iptables -A OUTPUT -p tcp --dport 80 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 8080 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 443 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 20 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 21 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 22 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 25 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 110 -j ACCEPT",
                "iptables -A OUTPUT -p tcp --dport 143 -j ACCEPT",
                "iptables -A OUTPUT -p udp --dport 53 -j ACCEPT",
                "iptables -A OUTPUT -p udp --dport 123 -j ACCEPT",
            ];
        }

        _ => {
            standard_messages(
                "warning",
                &format!("Option {} not found", option),
                "firewall_block",
                "cute",
            );
            return false;
        }
    }

    for rule in rules {
        let instance = ProcessInit {
            source: rule,
            source_from: "firewall_kill",
            source_description: "Set firewall rules",
            debug: debug,
        };
        result = system_command_exec(instance);
    }

    return result;
}

pub fn firewall_backup(path: &str, option: &str, debug: bool) -> bool {
    let mut command = "";

    match option {
        "backup" => {
            command = "iptables-save";
        }

        "restore" => {
            command = "iptables-restore";
        }

        _ => {
            standard_messages(
                "warning",
                &format!("Option {} not found", option),
                "firewall_block",
                "cute",
            );
            return false;
        }
    }

    let command_string = format!("{} {}/iptables_backup.ipbk", command, path);

    let instance = ProcessInit {
        source: command_string.as_str(),
        source_from: "firewall_backup",
        source_description: "Set Backup and Restore ip tables rules",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn firewall(ruleset: SimpleRule, debug: bool) -> bool {
    let rule = format!(
        "iptable -A {} -p {} --dport {} -j {}",
        ruleset.chain, ruleset.protocol, ruleset.destination_port, ruleset.table
    );

    let instance = ProcessInit {
        source: &rule,
        source_from: "firewall_kill",
        source_description: "Set firewall rules",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn shell_firewall(system_input:  &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--preset" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let option = &take_system_args(system_input, "--option");
            firewall_preset(option, debug)
        }

        "--backup" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let option = &take_system_args(system_input, "--option");
            let path = &take_system_args(system_input, "--path");

            firewall_backup(path, option, debug)
        }

        "--rule" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let command = SimpleRule {
                table: &take_system_args(system_input, "--table"),
                chain: &take_system_args(system_input, "--chain"),
                protocol: &take_system_args(system_input, "--protocol"),
                destination_port: &take_system_args(system_input, "--port"),
            };

            firewall(command, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_firewall", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
