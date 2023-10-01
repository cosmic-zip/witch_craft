### Functions

    use crate::core::utils::*;

    pub fn sample(generic_list: Vec<String>, debug: bool) -> bool {
        let cmd: String = format!("{}", "variable");
        if debug == true {
            system_text(&cmd, "yellow");
        }
        system_text("[VOID] :: Menssage", "green");
        system_command_exec(&cmd, debug)
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