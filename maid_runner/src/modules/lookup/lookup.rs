use crate::core::utils::*;
use crate::meow::meow::read_meow;
use crate::modules::lookup::lookup_structs::*;

pub fn lookup_mac_address(mac_address: LookupMacAddress, debug: bool) -> bool {
    let mut file: String;

    if mac_address.list_path == "default" {
        let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
        file = format!("{}{}", config["GENRAL_BASE_PATH"], config["MACADDR"]);
    } else {
        file = mac_address.list_path.to_string();
    }

    let cmd: String = format!("grep {} {}", mac_address.vendor_mac, file);
    if debug == true {
        system_text(&cmd, "yellow");
    }
    system_text("[LOOKUP_MAC_ADDRESS] :: Lookup mac address", "green");
    system_command_exec(&cmd, debug)
}

pub fn lookup_reverse_engineering(sample: LookupGenericPathOpType, debug: bool) -> bool {
    match sample.op_type {
        "s" => {
            system_text("[LOOKUP_FILE] :: Lookup strings", "green");
            let cmd = format!("strings {}", sample.sample_path);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "h" => {
            system_text("[LOOKUP_FILE] :: Lookup hexadecimal", "green");
            let cmd = format!("xxd {}", sample.sample_path);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "b" => {
            system_text("[LOOKUP_FILE] :: Lookup binary", "green");
            let cmd = format!("xxd -b {}", sample.sample_path);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "r" => {
            system_text("[LOOKUP_FILE] :: Lookup *todo*", "green");
            let cmd = format!("xxd -r {}", sample.sample_path);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "l" => {
            system_text("[LOOKUP_FILE] :: Lookup linked library", "green");
            let cmd = format!("ldd -v {}", sample.sample_path);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }
        _ => system_text(
            "struct LookupGenericPathOpType at op_type => Option not found",
            "yellow",
        ),
    }
}

pub fn lookup_exif_metadata(image: LookupGenericPathOpType, debug: bool) -> bool {
    let cmd = format!("exiftool {}", image.sample_path);
    if debug == true {
        system_text(&cmd, "yellow");
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
            system_text(
                "[USER ERROR] :: Invalid user input at → shell_lookup",
                "yellow",
            );
            return false;
        }
    }
}
