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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
