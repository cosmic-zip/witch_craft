use crate::core::core::*;
use crate::core::messages::standard_messages;
use crate::core::utils::*;
use crate::modules::scanner::scanner_structs::*;
use crate::read_meow;

pub fn scanner_web(source: ScannerWebGenericInput, debug: bool) -> bool {
    match source.op_type {
        "ip" => {
            let cmd = format!("ping -c4 {}", source.target);
            let msg = "Scanning ping";
            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", "Ping", &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "whois" => {
            let cmd = format!("whois {}", source.target);
            let msg = "Scanning whois";
            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", "Whois", &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "routes" => {
            let cmd = format!("traceroute -I {}", source.target);
            let msg = "Scanning traceroute ICMP";
            standard_messages("flaged", msg, "", "cute");

            if debug == true {
                standard_messages("debug", "Traceroute ICMP", &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug);

            let cmd = format!("traceroute -T {}", source.target);
            let msg = "Scanning traceroute TCP";
            standard_messages("flaged", msg, "", "cute");

            if debug == true {
                standard_messages("debug", "Traceroute TCP", &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug);

            let cmd = format!("traceroute -U {}", source.target);
            let msg = "Scanning traceroute UDP";
            standard_messages("flaged", msg, "", "cute");

            if debug == true {
                standard_messages("debug", "Traceroute UDP", &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "dns" => {
            let cmd = format!("dig +nocmd {} ANY +multiline +noquestion", source.target);
            let msg = "Searching for DNS records";
            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "nmap_tcp" => {
            let cmd = format!("nmap -sS -A -T3 {}", source.target);
            let msg = "Executing nmap TCP automation";
            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "nmap_udp" => {
            let cmd = format!("nmap -sU -T3 {}", source.target);
            let msg = "Executing nmap UDP automation";
            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "sub_domains" => {
            let path = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
            let final_path = &path["REPORT_BASE_PATH"];

            let cmd = format!(
                "dnsenum --enum {} -v -o {}mad_runner_sub_domains_{}.opsec -t 15 --threads 4 -f {}",
                source.target,
                final_path,
                system_time(),
                source.list_path,
            );
            let msg = "Scanning sub domains";

            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "sub_directories" => {
            let path = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
            let final_path = &path["REPORT_BASE_PATH"];

            system_text("[ROUTES] :: Sub directory", "green");
            let cmd = format!(
                "dirb {} {} -o {}maid_runner_sub_directories.opsec",
                source.target, source.list_path, final_path,
            );
            let msg = "Scanning sub domains";

            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "build_with" => {
            let cmd = format!("ping -c4 {}", source.target);
            let msg = "Buildwith search for possible frameworks";

            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "c_api_url" => {
            let cmd = format!("ping -c4 {}", source.target);
            let msg = "Curl scanning API endpoints";

            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        "c_web_url" => {
            let cmd = format!("ping -c4 {}", source.target);
            let msg = "Curl scanning web site url";
            standard_messages("flaged", msg, "", "cute");
            if debug == true {
                standard_messages("debug", msg, &cmd, "cute");
            }
            system_command_exec(&cmd, msg, debug)
        }

        _ => {
            let at = format!("{}", source.op_type);
            standard_messages(
                "warning",
                "Option not found on struct ScannerWebGenericInput",
                &at,
                "cute",
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
    }
    sport = source.ports;

    let cmd = format!("nmap {} {} -A -p {} {}", stype, stime, sport, source.target);
    let msg = "Executing nmap automation";
    standard_messages("flaged", msg, "", "cute");
    system_command_exec(&cmd, msg, debug)
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
