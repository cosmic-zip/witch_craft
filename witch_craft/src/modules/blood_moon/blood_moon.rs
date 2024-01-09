use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::blood_moon::blood_moon_structs::*;

pub fn blood_moon(data: SampleData, debug: bool) -> bool {
    let instance = ProcessInit {
        source: data.some,
        source_from: "blood_moon_module",
        source_description: "Set blood_moon rules",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn shell_blood_moon(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--blood_moon" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let blood_moon_data = SampleData {
                data: &take_system_args(system_input, "--data"),
            };
            blood_moon(blood_moon_data, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_blood_moon", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
