mod modules;

use crate::modules::core::core::*;
use crate::modules::core::data::*;
use crate::modules::watch::watch::*;


fn main() {

    let data = data();
    let argsv = readargs();
    let mname = argsv[1].as_str();

    if mname == "dns" {
        dns(argsv.clone());
    }

    for set in data {
        if set.name == mname {
            bob(set, argsv.clone());
        }
    }
}

#[cfg(test)]
mod test;
