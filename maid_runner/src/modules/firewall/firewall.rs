use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::firewall::firewall_structs::*;

pub fn firewall(option: &str, debug: bool) -> bool {

    match option {

        "kill" => {
            let instance = ProcessInit {
                source: &"sudo iptables -F && sudo iptables -P INPUT DROP && sudo iptables -P FORWARD DROP && sudo iptables -P OUTPUT DROP",
                source_from: "firewall_kill",
                source_description: "Set firewall rules",
                debug: debug,
            };
            system_command_exec(instance)
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

}

pub fn shell_lookup(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--firewall" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let option = &gsv(system_input.clone(), "--option");
            
            firewall(option, debug)         
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_firewall", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
