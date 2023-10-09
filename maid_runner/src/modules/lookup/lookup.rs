use crate::core::utils::*;
use crate::meow::meow::read_meow;
use crate::modules::lookup::lookup_structs::*;
use crate::core::messages::standard_messages;

pub fn lookup_mac_address(mac_address: LookupMacAddress, debug: bool) -> bool {
    let mut file: String;

    if mac_address.list_path == "default" {
        let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
        file = format!("{}{}", config["GENRAL_BASE_PATH"], config["MACADDR"]);
    } else {
        file = mac_address.list_path.to_string();
    }

    if debug == true {
        let message = format!("pattern: {}, path: {}", mac_address.vendor_mac, file,);
        standard_messages("debug", "Lookup mac address", &message, "cute");
    }

    standard_messages("falged", "Lookup mac address", "", "cute");
    
    match find_all_matching_lines(&file, mac_address.vendor_mac) {
        Ok(result) => {
            if !result.is_empty() {
                for line in result {
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
            standard_messages("error", "Error while lookup for mac address", &message, "cute");
            return false;
        }
    }
}

pub fn lookup_reverse_engineering(sample: LookupGenericPathOpType, debug: bool) -> bool {
    match sample.op_type {
        "s" => {
            standard_messages("flaged", "Lookup strings", "", "cute");
            let cmd = format!("strings {}", sample.sample_path);
            if debug == true {
                standard_messages("debug", "Lookup strings", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "h" => {
            standard_messages("flaged", "Lookup hexadecimal", "", "cute");
            let cmd = format!("xxd {}", sample.sample_path);
            if debug == true {
                standard_messages("debug", "ookup hexadecimal", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "b" => {
            standard_messages("flaged", "Lookup binary", "", "cute");
            let cmd = format!("xxd -b {}", sample.sample_path);
            if debug == true {
                standard_messages("debug", "Lookup binary", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "r" => {
            standard_messages("flaged", "Lookup *todo*", "", "cute");
            let cmd = format!("xxd -r {}", sample.sample_path);
            if debug == true {
                standard_messages("debug", "Lookup ", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "l" => {
            standard_messages("flaged", "Lookup linked library", "", "cute");
            let cmd = format!("ldd -v {}", sample.sample_path);
            if debug == true {
                standard_messages("debug", "Lookup ", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }
        _ => {
            let at = format!("{}", sample.op_type);
            standard_messages("warning", "Option not found on struct LookupGenericPathOpType", &at, "cute");
            return false;
        }
        
    }
}

pub fn lookup_exif_metadata(image: LookupGenericPathOpType, debug: bool) -> bool {
    let cmd = format!("exiftool {}", image.sample_path);
    if debug == true {
        standard_messages("warning", "Exiftool", &cmd, "cute");
    }
    system_command_exec(&cmd, debug)
}

pub fn shell_lookup(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--mac_address" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = LookupMacAddress {
                vendor_mac: &gsv(system_input.clone(), "--mac"),
                list_path: &gsv(system_input.clone(), "--path"),
            };

            lookup_mac_address(instance, debug)
        }

        "--lookup_metadata" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = LookupGenericPathOpType {
                sample_path: &gsv(system_input.clone(), "--sample"),
                op_type: "none",
            };

            lookup_exif_metadata(instance, debug)
        }

        "--lookup_reverse_engineering" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = LookupGenericPathOpType {
                sample_path: &gsv(system_input.clone(), "--sample"),
                op_type: &gsv(system_input.clone(), "--type"),
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
