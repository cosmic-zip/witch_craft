mod modules;

use crate::modules::core::core::*;
use crate::modules::core::data::*;

fn main() {

    let data = data();
    let argsv = readargs();
    for set in data {
        if set.name == argsv[1] {
            let cmd = lazy_loop(&set.meta, argsv.clone());
            lazy_exec(cmd, true);
        }
    }
}

#[cfg(test)]
mod test;
