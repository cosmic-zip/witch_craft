use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::cubes::cubes_structs::*;

pub fn cubes(data: Cubes, debug: bool) -> bool {

    let mut arch = "x86_64";

    if data.arch == "i386" {
        arch = "i386";
    }

    let commands = vec![
        format!("qemu-img create -f qcow2 {}.qcow2 {}", data.name, data.disk ),
        format!("qemu-system-{} -cdrom {} -drive file={}.qcow2 -m {} -smp 4",arch, data.path, data.name, data.ram),
    ];

    // if debug == true {
    //     println!("{:?}", &cubes);
    // }

    let mut bool_return = false; 
    for cmd in commands {
        let instance = ProcessInit {
            source: &cmd,
            source_from: "cubes_module",
            source_description: "Set cubes rules",
            debug: debug,
        };
    
        bool_return = system_command_exec(instance);
        if bool_return == false {
            return false;
        }
    }

    return bool_return

}

pub fn shell_cubes(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--qemu" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let cubes_data = Cubes {
                arch: &take_system_args(system_input, "--arch"),
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
