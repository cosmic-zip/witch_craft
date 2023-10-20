### Functions

    use crate::core::utils::*;

    pub fn sample(generic_list: Vec<String>, debug: bool) -> bool {
        let msg: &str = "simple message about this thing here";
        let cmd: String = format!("{}", "variable");
        if debug == true {
            system_text(&cmd, "yellow");
        }
        system_text("[VOID] :: Menssage", "green");
        system_command_exec(&cmd, &msg, debug)
    }

### Shell integracion

    pub fn shell_sample(system_input: Vec<String>) -> bool {
        
        let cmd_arg_name = system_input[2].as_str();

        match cmd_arg_name {
            "--sample" => {
                let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
                let instance = &gsv(system_input.clone(), "--sample");

                example_function_for_sample(instance, debug)
            }

            _ => {
                system_text(                    "🔴 [USER ERROR] :: Invalid user input at → shell_sample",
                    "yellow",
                );
                return false;
            }
        }
    }

### Pattern maching 

    pub fn find_all_matching_lines_usage() {
        let file_path = "path_to_your_file.txt"; // Replace with the path to your file
        let pattern = "your_pattern"; // Replace with the pattern you want to search

        match find_all_matching_lines(malware_db, pattern) {
            Ok(result) => {
                if !result.is_empty() {
                    for line in result {
                        println!("🚧 {}", line);
                    }
                    true
                } else {
                    println!("🔴 [WARNING] :: Pattern not found in any line.");
                    false
                }
            }
            Err(err) => {
                eprintln!("🔴 [ERROR] :: {}", err);
                false
            }
        }
    }