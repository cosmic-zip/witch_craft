use colored::*;

pub fn system_text(text: &str, color: &str) -> bool {
    let mut lines = text.lines();
    loop {
        match lines.next() {
            Some(line) => {
                if color == "black" {
                    println!("{}", line.black())
                } else if color == "red" {
                    println!("{}", line.red())
                } else if color == "green" {
                    println!("{}", line.green())
                } else if color == "yellow" {
                    println!("{}", line.yellow())
                } else if color == "blue" {
                    println!("{}", line.blue())
                } else if color == "purple" {
                    println!("{}", line.purple())
                } else if color == "cyan" {
                    println!("{}", line.cyan())
                } else {
                    println!("{}", line.white())
                }
            }
            None => break,
        }
    }
    println!("");
    return true;
}

fn capitalize_first_letter(s: &str) -> String {
    if let Some(first_char) = s.chars().next() {
        let mut capitalized = String::with_capacity(s.len());
        capitalized.push_str(&first_char.to_uppercase().collect::<String>());
        capitalized.push_str(&s[1..].to_lowercase());
        capitalized
    } else {
        String::new()
    }
}

pub fn standard_messages(level: &str, message: &str, at: &str, cuteness: &str) -> String {
    let mut color = "cyan";
    let mut icon = "*";
    let mut f_message = String::new();
    let mut f_at = String::new();

    match level {
        "debug" => {
            icon = "🔍";
            color = "white";
        }
        "flagged" => {
            icon = "🔖";
            color = "white";
        }
        "saved" => {
            icon = "💾";
            color = "white";
        }
        "success" => {
            icon = "✨";
            color = "green";
        }
        "warning" => {
            icon = "🚧";
            color = "yellow";
        }
        "error" => {
            icon = "🚨";
            color = "red";
        }
        "fatal" => {
            icon = "🔥";
            color = "cyan";
        }
        _ => {
            icon = "🏮";
            color = "white";
        }
    }

    if cuteness != "cute" {
        icon = "❱";
    }

    if message == "" {
        f_message = format!("🚧 [WARNING] :: None information givem :: at → standard_messages");
        system_text(&f_message, "yellow");
        return f_message;
    } else if level == "" {
        f_message = format!("🚧 [WARNING] :: Message level not givem :: at → standard_messages");
        system_text(&f_message, "yellow");
        return f_message;
    }

    if at != "" {
        f_at = format!(" :: at → {}", at);
    } else if at.len() > 0 {
        if at.chars().collect::<Vec<_>>()[0] != ' ' {
            f_at = format!(" :: at → {}", at);
        }
    } else {
        f_at = "".to_string();
    }

    let f_message = format!(
        "{} [{}] :: {}{}",
        icon,
        level.to_uppercase(),
        capitalize_first_letter(&message),
        capitalize_first_letter(&f_at)
    );

    system_text(&f_message, color);
    return f_message;
}
