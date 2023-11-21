use crate::modules::iso::iso_builder::*;
use crate::modules::iso::iso_structs::*;
use crate::core::structs::ProcessInit;
use crate::core::messages::standard_messages;
use crate::core::utils::*;

pub fn iso27x_build(data: CybersecurityFramework, debug: bool) -> bool {
    
    if debug == true {
        println("{:?}", data);
        return true;
    }

    return true;

}

pub fn shell_lookup(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {

        "--iso" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = CybersecurityFramework {
                data: &gsv(system_input.clone(), "--sample"),
            };

            iso27x_build(instance, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_sample", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
