#![allow(unused_variables)]
#![allow(dead_code)]

mod modules;

use crate::modules::core::core::*;
use crate::modules::core::data::*;

fn main() {
    let data = vec![
        DataSet::from_str("list", "ss -tupanr"),
    ];

    let argsv = readargs();
    for set in data {
        if set.name == argsv[1] {
            let cmd = lazy_loop(&set.meta, argsv.clone());
            let out = lazy_exec(cmd, true);
        }
    }
}

#[cfg(test)]
mod test;
