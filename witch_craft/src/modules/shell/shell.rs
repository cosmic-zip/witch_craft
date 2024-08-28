use crate::modules::binds::binds::*;
use crate::modules::core::consts::*;
use crate::modules::core::core::*;
use crate::modules::core::data::*;

pub fn flawless_entry_point(argsv: &[String]) -> i32 {
    let mname = argsv[1].as_str();
    let data = data();
    for set in data {
        if set.name == mname {
            let out = flawless_exec(set.clone(), argsv);
            if out != 0 {
                raise("Shell falied to execute at flawless_exec()", 4);
                raise(&set.meta, 4);
                return out;
            }
        }
    }

    return 42;
}

pub fn shell() -> i32 {
    // `args` is a slice of `String` values. The first item is the path to the binary,
    // and the subsequent items are arguments.
    // There are two valid cases:
    // 1. Executing something without arguments
    // 2. Executing something with arguments in the form of "--arg_key_name value"
    // In all cases, valid `args` will be provided.

    let argsv = readargs();

    if argsv.len() % 2 != 0 {
        println!("{}", WITCH);
        return 42;
    }

    match argsv[1].as_str() {
        "help" => magic_docs(),
        "dos.simple" => dos_simple_get_span(&argsv),
        "dos.longpw" => dos_long_auth_span(&argsv),
        "map.dns" => map_dns(&argsv),
        "blackcat.av" => blackcat_av(&argsv),
        _ => flawless_entry_point(&argsv),
    }
}
