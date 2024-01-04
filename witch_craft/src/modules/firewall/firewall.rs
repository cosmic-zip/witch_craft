use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::firewall::firewall_structs::NfTableRule;
use std::mem;

pub fn firewall_preset(option: &str, debug: bool) -> bool {
    let mut rules = Vec::new();
    let mut result: bool = false;

    match option {
        "reset" => {
            rules = vec!["nft flush ruleset"];
        }

        "kill" => {
            rules = vec![
                "nft flush ruleset",
                "nft add table inet filter",
                r"nft add chain inet filter input { type filter hook input priority 0 \; }",
                r"nft add chain inet filter output { type filter hook output priority 0 \; }",
                r"nft add chain inet filter forward { type filter hook forward priority 0 \; }",
            ];
        }

        "hardned" => {
            rules = vec![
                "nft flush ruleset",
                "nft add table inet filter",
                r"nft add chain inet filter input { type filter hook input priority 0 \; }",
                r"nft add chain inet filter output { type filter hook output priority 0 \; }",
                r"nft add chain inet filter forward { type filter hook forward priority 0 \; }",
                "nft add rule inet filter output tcp dport 80 accept",
                "nft add rule inet filter output tcp dport 8080 accept",
                "nft add rule inet filter output tcp dport 443 accept",
                "nft add rule inet filter output tcp dport 20 accept",
                "nft add rule inet filter output tcp dport 21 accept",
                "nft add rule inet filter output tcp dport 22 accept",
                "nft add rule inet filter output tcp dport 25 accept",
                "nft add rule inet filter output tcp dport 110 accept",
                "nft add rule inet filter output tcp dport 143 accept",
                "nft add rule inet filter output udp dport 53 accept",
                "nft add rule inet filter output udp dport 123 accept",
                "nft add rule inet filter input drop",
                "nft add rule inet filter forward drop",
                "nft add rule inet filter output drop",
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

pub fn firewall(ruleset: NfTableRule, debug: bool) -> bool {
    let rule = format!(
        "nft add rule inet filter output {} dport {} {}",
        ruleset.protocol, ruleset.port, ruleset.action
    );

    let instance = ProcessInit {
        source: &rule,
        source_from: "firewall_kill",
        source_description: "Set firewall rules",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn shell_firewall(system_input: &mut Vec<String>) -> bool {
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
            let command = NfTableRule {
                protocol: &take_system_args(system_input, "--protocol"),
                port: &take_system_args(system_input, "--port"),
                action: &take_system_args(system_input, "--action"),
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
