// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn readrc_value(key: &str) -> String {
    let fkey = format!("{}=", key);
    let home = format!("{}.witchrc", get_os_env("HOME"));

    match fs::read_to_string(&home) {
        Ok(file) => {
            let lines: Vec<&str> = file.split("\n").collect();
            for line in lines {
                if line.contains(&fkey) {
                    return line.replace(&fkey, "");
                }
            }
            "".to_string()
        },
        Err(err) => {
            println!("{}-{}", err, home);
            return "error".to_string();
        }
    }
}

#[tauri::command]
fn read_json() -> String {
    let witchrc = readrc_value("path_log_file");
    let home = get_os_env("HOME");
    let path = witchrc.replace("~/", &home);

    fs::read_to_string(&path)
}



fn main() {
    witch_oracle_lib::run()
}
