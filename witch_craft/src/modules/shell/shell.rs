use crate::modules::core::consts::*;
use crate::modules::core::core::*;
use crate::modules::core::data::*;

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
        return 1;
    }

    let mname = argsv[1].as_str();

    match mname {
        "help" => {
            magic_docs();
        }
        _ => {
            let data = data();
            for set in data {
                if set.name == mname {
                    let out = flawless_exec(set.clone(), &argsv);
                    if out != 0 {
                        raise("Shell falied to execute at flawless_exec()", 4);
                        raise(&set.meta, 4);
                        return out;
                    }
                }
            }
        }
    }

    0
}
