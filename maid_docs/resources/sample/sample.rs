use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::sample::sample_structs::*;

pub fn sample(data: SampleData, debug: bool) -> bool {
    let instance = ProcessInit {
        source: data.some,
        source_from: "sample_module",
        source_description: "Set sample rules",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn shell_sample(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--sample" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let sample_data = SampleData {
                data: &take_system_args(system_input, "--data"),
            };
            sample(sample_data, debug)
        }
    }
}
