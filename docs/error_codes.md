system_text(&cmd, "yellow");
system_text(&cmd, "yellow");
system_text("[USER_ERROR] :: Invalid user input at → shell_core", "yellow",
system_text("[REPORT] :: Report created", "green");
pub fn system_text(text: &str, color: &str) -> bool {
use crate::core::utils::system_text;
"unix_network" => system_text(UNIX_NETWORK, "cyan"),
"unix_sys_info" => system_text(UNIX_SYSTEMINFO, "cyan"),
"unix_utility" => system_text(UNIX_UTILITY, "cyan"),
"unix_command" => system_text(UNIX_COMMAND, "cyan"),
"unix_misc" => system_text(UNIX_MISC, "cyan"),
"unix_files" => system_text(UNIX_FILES, "cyan"),
"unix_folders" => system_text(UNIX_FOLDERS, "cyan"),
"windows_files" => system_text(WINDOWS_FILES, "cyan"),
"windows_reg" => system_text(WINDOWS_COMMON_REGISTER_LOCATIONS, "cyan"),
"windows_cmd" => system_text(WINDOWS_CMD_BASICS, "cyan"),
"windows_powershell" => system_text(WINDOWS_POWERSHELL_BASICS, "cyan"),
"header" => system_text(MAID_RUNNER_HEADER, "purple"),
"maid" => system_text(MAID_RUNNER_BANNER, "white"),
system_text(MAID_RUNNER_HEADER, "purple");
system_text(MAID_RUNNER_MAIN_HELP, "purple");
system_text("[PING] :: Ping", "green");
system_text(&cmd, "yellow");
system_text("[WHOIS] :: Whois", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Traceroute ICMP", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Traceroute TCP", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Traceroute UDP", "green");
system_text(&cmd, "yellow");
system_text("[DNS] :: DNS", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Nmap TCP", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Nmap UDP", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Sub domains", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Sub directory", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: Buildwith", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: API url scanner", "green");
system_text(&cmd, "yellow");
system_text("[ROUTES] :: WEB url scanner", "green");
system_text(&cmd, "yellow");
system_text( "struct ScannerWebGenericInput at op_type → Option not found",
system_text("[AUTO_MAP] :: EXEC nmap automation", "green");
system_text("[USER ERROR] :: Invalid user input at → shell_scanner",
system_text("[CURL_BIND] :: Curl binds", "green");
system_text(&cmd, "yellow");
system_text("[CURL_BIND] :: Curl header lookup", "green");
system_text(&cmd, "yellow");
system_text("[CURL_BIND] :: Curl request status_code", "green");
system_text(&cmd, "yellow");
system_text("[USER ERROR] :: Invalid user input at → shell_curl",
// system_text,
// system_command_exec, system_string_to_vec_builder, system_text, system_time,
system_text(&format!("🔴 [ERROR] :: file path {}", file_path), "red");
system_text("🔴 [ERROR] :: No matching rows found.", "red");
system_text(&cmd, "yellow");
system_text("🔶 [MALWARE_PATTERN] :: Searching for malware pattern",
system_text("🔴 [USER ERROR] :: Invalid user input at → shell_maid_av",
system_text(&cmd, "yellow");
system_text("[LOOKUP_MAC_ADDRESS] :: Lookup mac address", "green");
system_text("[LOOKUP_FILE] :: Lookup strings", "green");
system_text(&cmd, "yellow");
system_text("[LOOKUP_FILE] :: Lookup hexadecimal", "green");
system_text(&cmd, "yellow");
system_text("[LOOKUP_FILE] :: Lookup binary", "green");
system_text(&cmd, "yellow");
system_text("[LOOKUP_FILE] :: Lookup *todo*", "green");
system_text(&cmd, "yellow");
system_text("[LOOKUP_FILE] :: Lookup linked library", "green");
system_text(&cmd, "yellow");
_ => system_text("struct LookupGenericPathOpType at op_type => Option not found",
system_text(&cmd, "yellow");
system_text("[USER ERROR] :: Invalid user input at → shell_lookup",
system_text(&res.stdout, "green");
system_text(&res.stdout, "red");
system_text(&format!("[RAW_COMMAND] :: Trying exec → {}", cmd), "green");
