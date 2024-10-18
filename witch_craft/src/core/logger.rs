pub fn logger(cmd_output: String, cmd_true: String, cmd_witchy: String) -> bool {

    let now = datetime_now();

    if readrc_exists("enable_logger") {
        let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("my-file")
                .unwrap();

        if let Err(e) = writeln!(file, "{}", cmd_output) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    false
}
