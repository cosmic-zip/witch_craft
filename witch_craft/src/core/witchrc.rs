use std::fs;

pub fn readrc_exists(key: &str) -> bool {
    let fkey = format!("{}=", key);

    match fs::read_to_string("~/.witchrc") {
        Ok(file) => {
            let lines: Vec<&str> = file.split("\n").collect();
            for line in lines {
                if line.contains(&fkey) {
                    return true;
                }
            }
            false
        },
        Err(err) => {
            eprintln!("{}", err);
            return false;
        }
    }
}

pub fn readrc_value(key: &str) -> String {
    let fkey = format!("{}=", key);

    match fs::read_to_string("~/.witchrc") {
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
            eprintln!("{}", err);
            return "error".to_string();
        }
    }
}
