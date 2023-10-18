use crate::read_meow;
use crate::core::core::*;
use crate::core::utils::*;
use crate::core::messages::standard_messages;
use crate::modules::scanner::scanner_structs::*;

pub fn scanner_web(source: ScannerWebGenericInput, debug: bool) -> bool {
    match source.op_type {
        "ip" => {
            standard_messages("flaged", "Ping", "", "cute");
            let cmd = format!("ping -c4 {}", source.target);
            if debug == true {
                standard_messages("debug", "Ping", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "whois" => {
            standard_messages("flaged", "Whois", "", "cute");
            let cmd = format!("whois {}", source.target);
            if debug == true {
                standard_messages("debug", "Whois", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "routes" => {
            standard_messages("flaged", "Traceroute ICMP", "", "cute");
            let cmd = format!("traceroute -I {}", source.target);
            if debug == true {
                standard_messages("debug", "Traceroute ICMP", &cmd, "cute");
            }
            system_command_exec(&cmd, debug);

            standard_messages("flaged", "Traceroute TCP", "", "cute");
            let cmd = format!("traceroute -T {}", source.target);
            if debug == true {
                standard_messages("debug", "Traceroute TCP", &cmd, "cute");
            }
            system_command_exec(&cmd, debug);

            standard_messages("flaged", "Traceroute UDP", "", "cute");
            let cmd = format!("traceroute -U {}", source.target);
            if debug == true {
                standard_messages("debug", "Traceroute UDP", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "dns" => {
            standard_messages("flaged", "DNS", "", "cute");
            let cmd = format!("dig +nocmd {} ANY +multiline +noquestion", source.target);
            if debug == true {
                standard_messages("debug", "DNS", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "nmap_tcp" => {
            standard_messages("flaged", "Nmap TCP", "", "cute");
            let cmd = format!("nmap -sS -A -T3 {}", source.target);
            if debug == true {
                standard_messages("debug", "Nmap TCP", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "nmap_udp" => {
            standard_messages("flaged", "Nmap UDP", "", "cute");
            let cmd = format!("nmap -sU -T3 {}", source.target);
            if debug == true {
                standard_messages("debug", "Nmap UDP", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "sub_domains" => {
            let path = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
            let final_path = &path["REPORT_BASE_PATH"];

            system_text("[ROUTES] :: Sub domains", "green");
            let cmd = format!(
                "dnsenum --enum {} -v -o {}mad_runner_sub_domains_{}.opsec -t 15 --threads 4 -f {}",
                source.target,
                final_path,
                system_time(),
                source.list_path,
            );
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "sub_directories" => {

            let path = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
            let final_path = &path["REPORT_BASE_PATH"];

            system_text("[ROUTES] :: Sub directory", "green");
            let cmd = format!(
                "dirb {} {} -o {}maid_runner_sub_directories.opsec",
                source.target, source.list_path, final_path,
            );
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "build_with" => {
            system_text("[ROUTES] :: Buildwith", "green");
            let cmd = format!("ping -c4 {}", source.target);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "c_api_url" => {
            system_text("[ROUTES] :: API url scanner", "green");
            let cmd = format!("ping -c4 {}", source.target);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        "c_web_url" => {
            system_text("[ROUTES] :: WEB url scanner", "green");
            let cmd = format!("ping -c4 {}", source.target);
            if debug == true {
                system_text(&cmd, "yellow");
            }
            system_command_exec(&cmd, debug)
        }

        _ => {
            system_text(
                "struct ScannerWebGenericInput at op_type → Option not found",
                "yellow",
            );
            false
        }
    }
}

pub fn scanner_auto_nmap(source: ScannerWebAutoNmap, debug: bool) -> bool {
    let mut stime: &str = "";
    let mut stype: &str = "";
    let mut sport: &str = "";

    if "fast" == source.delay {
        stime = "T4";
    } else if "slow" == source.delay {
        stime = "T2";
    } else if "paranoid" == source.delay {
        stime = "T0";
    } else {
        return false;
    }

    if source.scan_type == "tcp_syn" {
        stype = "-sS";
    } else if source.scan_type == "tcp_ack" {
        stype = "-sA";
    } else if source.scan_type == "tcp_null" {
        stype = "-sN";
    } else if source.scan_type == "udp" {
        stype = "-sU";
    } else if source.scan_type == "connect" {
        stype = "-sT";
    } else if source.scan_type == "widnows" {
        stype = "-sW";
    } else if source.scan_type == "maimon" {
        stype = "-sM";
    } else if source.scan_type == "fin" {
        stype = "-sF";
    } else if source.scan_type == "xmas" {
        stype = "-sX";
    }

    if source.ports == "auto" {
        sport = NMAP_PORTS;
    } else if source.ports == "all" {
        sport = "-p-";
    } else {
        sport = source.ports;
    }

    let cmd = format!("nmap {} {} -A -p {} {}", stype, stime, sport, source.target);
    system_text("[AUTO_MAP] :: EXEC nmap automation", "green");
    system_command_exec(&cmd, debug)
}

pub fn shell_scanner(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--scanner_web" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = ScannerWebGenericInput {
                target: &gsv(system_input.clone(), "--target"),
                op_type: &gsv(system_input.clone(), "--type"),
                list_path: &gsv(system_input.clone(), "--path"),
            };

            scanner_web(instance, debug)
        }

        "--scanner_auto_nmap" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = ScannerWebAutoNmap {
                target: &gsv(system_input.clone(), "--target"),
                delay: &gsv(system_input.clone(), "--delay"),
                ports: &gsv(system_input.clone(), "--ports"),
                scan_type: &gsv(system_input.clone(), "--type"),
            };

            scanner_auto_nmap(instance, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_scanner", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
