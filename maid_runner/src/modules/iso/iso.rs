use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::iso::iso_builder::*;
use crate::modules::iso::iso_structs::*;

pub fn iso27x_build(data: CybersecurityFramework, debug: bool) -> bool {
    if debug == true {
        println!("{:?}", data);
        return true;
    }

    return true;
}

pub fn shell_iso(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--iso" => true,

        _ => {
            standard_messages("warning", "Invalid user input", "shell_sample", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
