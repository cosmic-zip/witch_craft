use crate::core::core::*;
use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::scanner::scanner_structs::*;
use crate::read_meow;

pub fn scanner_web(source: ScannerWebGenericInput, debug: bool) -> bool {
    match source.op_type {
        "ip" => {
            let instance = ProcessInit {
                source: &format!("ping -c4 {}", source.target),
                source_from: "scanner",
                source_description: "Scanning ip address",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "whois" => {
            let instance = ProcessInit {
                source: &format!("whois {}", source.target),
                source_from: "scanner",
                source_description: "Scanning whois",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "routes" => {
            let instance = ProcessInit {
                source: &format!("traceroute -I {}", source.target),
                source_from: "scanner",
                source_description: "Scanning traceroute ICMP",
                debug: debug,
            };
            system_command_exec(instance);

            let instance = ProcessInit {
                source: &format!("traceroute -T {}", source.target),
                source_from: "scanner",
                source_description: "Scanning traceroute TCP",
                debug: debug,
            };
            system_command_exec(instance);

            let instance = ProcessInit {
                source: &format!("traceroute -U {}", source.target),
                source_from: "scanner",
                source_description: "Scanning traceroute UDP",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "dns" => {
            let instance = ProcessInit {
                source: &format!("dig +nocmd {} ANY +multiline +noquestion", source.target),
                source_from: "scanner",
                source_description: "Searching for DNS records",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "nmap_tcp" => {
            let instance = ProcessInit {
                source: &format!("nmap -sS -A -T3 {}", source.target),
                source_from: "scanner",
                source_description: "Executing nmap TCP automation",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "nmap_udp" => {
            let instance = ProcessInit {
                source: &format!("nmap -sS -A -T3 {}", source.target),
                source_from: "scanner",
                source_description: "Executing nmap UDP automation",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "sub_domains" => {
            let path = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
            let final_path = &path["REPORT_BASE_PATH"];

            let command = format!(
                "dnsenum --enum {} -v -o {}mad_runner_sub_domains_{}.opsec -t 15 --threads 4 -f {}",
                source.target,
                final_path,
                system_time(),
                source.list_path,
            );

            let instance = ProcessInit {
                source: &command,
                source_from: "scanner",
                source_description: "Scanning sub domains",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "sub_directories" => {
            let path = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
            let final_path = &path["REPORT_BASE_PATH"];

            let command = format!(
                "dirb {} {} -o {}maid_runner_sub_directories.opsec",
                source.target, source.list_path, final_path,
            );

            let instance = ProcessInit {
                source: &command,
                source_from: "scanner",
                source_description: "Scanning sub directories",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "build_with" => {
            let instance = ProcessInit {
                source: "DUMP_TBV",
                source_from: "scanner",
                source_description: "Scanning sub directories",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "c_api_url" => {
            let instance = ProcessInit {
                source: "DUMP_TBV",
                source_from: "scanner",
                source_description: "Scanning sub directories",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "c_web_url" => {
            let instance = ProcessInit {
                source: "DUMP_TBV",
                source_from: "scanner",
                source_description: "Scanning sub directories",
                debug: debug,
            };
            system_command_exec(instance)
        }

        _ => {
            standard_messages(
                "warning",
                "Option not found on struct ScannerWebGenericInput",
                &format!("{}", source.op_type),
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
        standard_messages(
            "warning",
            "Invalid user input",
            "Options are: fast, slow and paranoid",
            "cute",
        );
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
    } else if source.scan_type == "window" {
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

    if debug == true {
        println!("{}", source.target);
    }

    let instance = ProcessInit {
        source: &format!("nmap {} {} -A -p {} {}", stype, stime, sport, source.target),
        source_from: "scanner",
        source_description: "Executing nmap automation",
        debug: debug,
    };
    system_command_exec(instance)
}

pub fn shell_scanner(system_input:  &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--scanner_web" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = ScannerWebGenericInput {
                target: &take_system_args(system_input, "--target"),
                op_type: &take_system_args(system_input, "--type"),
                list_path: &take_system_args(system_input, "--path"),
            };

            scanner_web(instance, debug)
        }

        "--scanner_auto_nmap" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = ScannerWebAutoNmap {
                target: &take_system_args(system_input, "--target"),
                delay: &take_system_args(system_input, "--delay"),
                ports: &take_system_args(system_input, "--ports"),
                scan_type: &take_system_args(system_input, "--type"),
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
