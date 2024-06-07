use crate::modules::core::core::*;
use crate::modules::core::data::*;
use crate::modules::shell::fancy::*;
use crate::modules::binds::binds::*;

pub fn check() -> bool {

    let packages: Vec<&str> = vec![
        "nmap", "dirb", "dnsenum", "ldd", "xxd", "iptables",
        "ss", "stat", "wget", "curl", "dig"
    ];

    for pkg in packages {
        let out = lazy_exec(pkg.to_string(), true);
        // code 127 are for not found
        if out == 127 {
            raise(&format!("Fail! {}", pkg), 4);
            return false;
        }
    }

    raise("All checks pass!", 1);
    return true;

}


pub fn private_enable() {
    println!("{}", KEY);
}

pub fn shell() -> i32 {
    // Argsv are and Vec<String>, the first item are 
    // the path of binary, the rest are all arguments
    // them, exist only two valid cases:
    // witchcraft something (exec something that dont have args)
    //  witchcraft something --arg_key_name value
    // In all cases an valid ARGSV are going to be iven

    let argsv = readargs();

    if argsv.len() % 2 != 0 {
        raise("Not enough parameters provided.", 3);
        println!("{}", WITCH); 
        return 1;
    }


    let data = data();
    let mname = argsv[1].as_str();

    match mname {
        "dns" => {
            dns(argsv.clone());
        },
        "check" => {
            check();
        },
        "private" => {
            private_enable();
        },
        "file.compact" => {
            plugin_file_compact(argsv.clone());
        },
        _ => {
            //
        }
    }

    for set in data {
        if set.name == mname {
            let out = bob(set, argsv.clone());
            if out != 0 { 
                raise("Shell falied to execute at bob()", 4); 
                return out;
            }
        }
    }

    return 0

}