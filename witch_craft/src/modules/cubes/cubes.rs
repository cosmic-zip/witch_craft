use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::cubes::cubes_structs::*;

pub fn cubes(data: Cubes, debug: bool) -> bool {

    let cubes = format!("qemu-system-x86_64 -cdrom {} -drive file={},size={} -m {}",
        data.path,
        data.name,
        data.disk,
        data.ram,
    );

    if debug == true {
        println!("{:?}", &cubes);
    }

    let instance = ProcessInit {
        source: &cubes,
        source_from: "cubes_module",
        source_description: "Set cubes rules",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn shell_cubes(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--cubes" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let cubes_data = Cubes {
                path: &take_system_args(system_input, "--path"),
                name: &take_system_args(system_input, "--name"),
                disk: &take_system_args(system_input, "--disk"),
                ram: &take_system_args(system_input, "--ram"),
            };
            cubes(cubes_data, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_cubes", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
