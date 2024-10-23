use crate::{core::consts::SYSINFO_MAID, get_os_env};
use std::process::Command;
use sysinfo::System;

pub fn maid_info(_argsv: &[String]) -> i32 {
    let sys = System::new_all();

    // CPU Load
    let cpu_load = format!("{:.2}%", sys.global_cpu_usage());

    // Desktop Environment
    let desktop_environment = get_os_env("XDG_CURRENT_DESKTOP");

    // Memory Stats
    let mem_total = format!("{}Mb", sys.total_memory() / 1048576);
    let mem_free = format!("{}Mb", sys.available_memory() / 1048576);

    // Network Status
    let net_status = "Up";

    // OS Name
    let os_name = format!(
        "{} :: {}",
        System::name().unwrap(),
        System::os_version().unwrap()
    );

    // Kernel Version
    let kernel_version = System::kernel_version().unwrap();

    // Shell
    let shell = get_os_env("SHELL");

    let mut text = SYSINFO_MAID.to_string();
    text = text.replace("@@@cpu_load", &format!("cpu load: {}", cpu_load));
    text = text.replace(
        "@@@desktop_environment",
        &format!("desktop_environment: {}", desktop_environment),
    );
    text = text.replace("@@@mem_total", &format!("mem total: {}", mem_total));
    text = text.replace("@@@mem_free", &format!("mem free: {}", mem_free));
    text = text.replace("@@@net_status", &format!("net status: {}", net_status));
    text = text.replace("@@@os_name", &format!("os name: {}", os_name));
    text = text.replace(
        "@@@kernel_version",
        &format!("kernel version: {}", kernel_version),
    );
    text = text.replace("@@@shell", &format!("shell: {}", shell));

    println!("{}", text);
    return 0;
}
