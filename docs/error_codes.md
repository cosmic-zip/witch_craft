## Actual menssages

system_text("[USER_ERROR] :: Invalid user input at → shell_core", "yellow",
system_text("[REPORT] :: Report created", "green");
system_text("[PING] :: Ping", "green");
system_text("[WHOIS] :: Whois", "green");
system_text("[ROUTES] :: Traceroute ICMP", "green");
system_text("[ROUTES] :: Traceroute TCP", "green");
system_text("[ROUTES] :: Traceroute UDP", "green");
system_text("[DNS] :: DNS", "green");
system_text("[ROUTES] :: Nmap TCP", "green");
system_text("[ROUTES] :: Nmap UDP", "green");
system_text("[ROUTES] :: Sub domains", "green");
system_text("[ROUTES] :: Sub directory", "green");
system_text("[ROUTES] :: Buildwith", "green");
system_text("[ROUTES] :: API url scanner", "green");
system_text("[ROUTES] :: WEB url scanner", "green");
system_text( "struct ScannerWebGenericInput at op_type → Option not found",
system_text("[AUTO_MAP] :: EXEC nmap automation", "green");
system_text("[USER ERROR] :: Invalid user input at → shell_scanner",
system_text("[CURL_BIND] :: Curl binds", "green");
system_text("[CURL_BIND] :: Curl header lookup", "green");
system_text("[CURL_BIND] :: Curl request status_code", "green");
system_text("[USER ERROR] :: Invalid user input at → shell_curl",
system_text(&format!("🔴 [ERROR] :: file path {}", file_path), "red");
system_text("🔴 [ERROR] :: No matching rows found.", "red");
system_text("🔶 [MALWARE_PATTERN] :: Searching for malware pattern",
system_text("🔴 [USER ERROR] :: Invalid user input at → shell_maid_av",
system_text("[LOOKUP_MAC_ADDRESS] :: Lookup mac address", "green");
system_text("[LOOKUP_FILE] :: Lookup strings", "green");
system_text("[LOOKUP_FILE] :: Lookup hexadecimal", "green");
system_text("[LOOKUP_FILE] :: Lookup binary", "green");
system_text("[LOOKUP_FILE] :: Lookup *todo*", "green");
system_text("[LOOKUP_FILE] :: Lookup linked library", "green");
system_text("struct LookupGenericPathOpType at op_type => Option not found",
system_text("[USER ERROR] :: Invalid user input at → shell_lookup",

## Error Messages 

Our error messages are organized into different categories based on their severity and impact on the user experience. We use a numbering system to categorize errors, with numbers between 1000 and 9000. Here's what each category means:

**Numbers between 1000 and 4000:** These error messages indicate issues that are close to the system or infrastructure. They may affect the overall performance of our platform, but they don't directly impact the user experience. Examples include network connectivity problems, server overload, or database queries taking too long to execute.

**Numbers between 4001 and 6000:** These error messages indicate issues that are slightly more critical than those in the previous category. They may affect specific features or functionality within our platform, but they don't compromise the overall system stability. Examples include authentication failures, data corruption during read/write, or unexpected behavior from a particular feature.

**Numbers between 6001 and 8000:** These error messages indicate issues that are moderately critical and may impact the user experience to some extent. They could be related to data loss, incorrect functionality, or other problems that require attention but don't pose an immediate threat to system stability. Examples include corrupted files, unexpected behavior from a specific feature, or temporary data loss during a save operation.

**Numbers between 8001 and 9000:** These error messages indicate the most critical issues that can significantly impact the user experience. They may involve security breaches, data loss, or other severe problems that require immediate attention to prevent further damage. Examples include unauthorized access attempts, data corruption during save operations, or system-wide failures that compromise platform stability.


**Numbers between 9001 and 9999:** These error messages indicate issues that are not critical or simple system messages, but may still be useful for debugging or troubleshooting purposes. They could be related to minor bugs, unexpected behavior from a feature, or other non-critical problems. Examples include unexpected output from a function, a minor bug in a specific feature, or an unhandled edge case.


## Common types of errors that can occur in a CLI:


- Access Denied
- Syntax Errors
- Command Not Found
- Parameter Validation Errors
- Insufficient Privileges
- File I/O Errors
- Network Connectivity Issues


Debug: Debug messages are used for troubleshooting and providing detailed information about the program's execution. They are typically used during development and debugging processes.

    Info: Informational messages provide general information about the program's operation. They are used to convey important details about the program's state or progress.

    Warning: Warning messages indicate potential issues or situations that might need attention but do not necessarily stop the program's execution. They serve as alerts for developers or users.

    Error: Error messages signify critical issues that prevent the program from functioning correctly. These messages usually lead to the termination of the program's execution.

    Fatal: Fatal messages represent severe errors that result in the program's immediate termination. They often indicate unrecoverable errors or critical system failures.