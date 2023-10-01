pub fn standart_messages(code: u32, level: &str, message: &str) {

    let mut f_message = String::new();

    if message != "" {
        f_message = format!(":: at → {}", message);
    }

    let color = match level {
        "info"      => "white",
        "fine"      => "green",
        "warning"   => "yellow",
        "error"     => "red",
    }

    let result = match rtype {
        1001 => format!("CODE :: 1001 Access Denied {}", f_message),
        1002 => format!("CODE :: 1002 File I/O Error {}", f_message),
        1003 => format!("CODE :: 1003 Command Not Found {}", f_message),
        1004 => format!("CODE :: 1004 Parameter Validation Error {}", f_message),
        4001 => format!("CODE :: 4001 Insufficient Privileges {}", f_message),
        4002 => format!("CODE :: 4002 Network Connectivity Issues {}", f_message),
        9001 => format!("CODE :: 9001 System Information {}", f_message),
    }

    system_text(&result, color);
    
}