use crate::modules::sample::sample_structs::*;
use crate::core::structs::ProcessInit;
use crate::core::messages::standard_messages;
use crate::core::utils::*;

pub fn sample(data: SampleData, debug: bool) -> bool {
    let instance = ProcessInit {
        source: &format!("foo {}", data.data),
        source_from: "sample",
        source_description: "Sample tool",
        debug: debug,
    };
    system_command_exec(instance)
}

pub fn shell_lookup(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {

        "--sample" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = SampleData {
                data: &gsv(system_input.clone(), "--sample"),
            };

            sample(instance, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_sample", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
