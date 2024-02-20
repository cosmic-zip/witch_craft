use crate::core::messages::standard_messages;
use crate::core::report::logger;
use crate::core::structs::Logger;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::meow::meow::read_meow;
use crate::modules::lookup::lookup_structs::*;

pub fn lookup_mac_address(mac_address: LookupMacAddress, debug: bool) -> bool {
    let file: String;

    if mac_address.list_path == "default" {
        let config = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
        file = format!("{}{}", config["GENERAL_BASE_PATH"], config["MACADDR"]);
    } else {
        file = mac_address.list_path.to_string();
    }

    if debug == true {
        let message = format!("pattern: {}, path: {}", mac_address.vendor_mac, file,);
        standard_messages("debug", "Lookup mac address", &message, "cute");
    }

    // standard_messages("falged", "Lookup mac address", "", "cute");

    match find_all_matching_lines(&file, mac_address.vendor_mac) {
        Ok(result) => {
            if !result.is_empty() {
                for line in result {
                    let data = Logger {
                        source: "lookup_mac_address".to_string(),
                        source_from: "lookup".to_string(),
                        source_description: "Lookup mac address".to_string(),
                        status: 0.to_string(),
                        stdout: format!("{:?}", line),
                        stderr: "none".to_string(),
                        debug: debug,
                    };

                    match logger(data) {
                        Ok(_result) => {
                            // standard_messages("saved", "Log saved", "", "cute");
                        }
                        Err(_err) => println!("error:src.modules.osint.city_geo_location"),
                    }

                    standard_messages("falged", "Found", &line, "cute");
                }

                return true;
            } else {
                standard_messages("warning", "Pattern not found in any line.", "", "cute");
                return false;
            }
        }
        Err(err) => {
            let message = format!("{}", err);
            standard_messages(
                "error",
                "Error while lookup for mac address",
                &message,
                "cute",
            );
            return false;
        }
    }
}

pub fn lookup_reverse_engineering(sample: LookupGenericPathOpType, debug: bool) -> bool {
    match sample.op_type {
        "s" => {
            let instance = ProcessInit {
                source: &format!("strings {}", sample.sample_path),
                source_from: "lookup",
                source_description: "Lookup strings",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "h" => {
            let instance = ProcessInit {
                source: &format!("xxd {}", sample.sample_path),
                source_from: "lookup",
                source_description: "Lookup hexadecimal",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "b" => {
            let instance = ProcessInit {
                source: &format!("xxd -b {}", sample.sample_path),
                source_from: "lookup",
                source_description: "Lookup binary",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "d" => {
            let instance = ProcessInit {
                source: &format!("stat {}", sample.sample_path),
                source_from: "lookup",
                source_description: "Lookup details properts",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "l" => {
            let instance = ProcessInit {
                source: &format!("ldd -v {}", sample.sample_path),
                source_from: "lookup",
                source_description: "Lookup linked library",
                debug: debug,
            };
            system_command_exec(instance)
        }
        _ => {
            standard_messages(
                "warning",
                "Option not found on struct LookupGenericPathOpType",
                &format!("{}", sample.op_type),
                "cute",
            );
            return false;
        }
    }
}

pub fn lookup_exif_metadata(image: LookupGenericPathOpType, debug: bool) -> bool {
    let instance = ProcessInit {
        source: &format!("exiftool {}", image.sample_path),
        source_from: "lookup",
        source_description: "Lookup exiftool",
        debug: debug,
    };
    system_command_exec(instance)
}

pub fn shell_lookup(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--mac_address" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = LookupMacAddress {
                vendor_mac: &take_system_args(system_input, "--mac"),
                list_path: &take_system_args(system_input, "--path"),
            };

            lookup_mac_address(instance, debug)
        }

        "--lookup_metadata" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = LookupGenericPathOpType {
                sample_path: &take_system_args(system_input, "--sample"),
                op_type: "none",
            };

            lookup_exif_metadata(instance, debug)
        }

        "--lookup_re" | "" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = LookupGenericPathOpType {
                sample_path: &take_system_args(system_input, "--sample"),
                op_type: &take_system_args(system_input, "--type"),
            };

            lookup_reverse_engineering(instance, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_lookup", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
