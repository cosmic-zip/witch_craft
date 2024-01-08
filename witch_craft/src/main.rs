#![allow(unused_imports)]
#![allow(unused_assignments)]

mod core;
mod meow;
mod modules;
mod shell;
mod manual;

use crate::manual::manual::*;
use crate::core::utils::*;
use crate::meow::meow::*;
use crate::modules::{
    attack::attack::*,
    attack::attack_structs::*,
    antivirus::antivirus::*,
    antivirus::antivirus_structs::*,
    botnet::botnet::*,
    botnet::botnet_structs::*,
    curl::curl::*,
    curl::curl_structs::*,
    firewall::firewall::*,
    firewall::firewall_structs::*,
    botnet::botnet::*,
    botnet::botnet_structs::*,
    lookup::lookup::*,
    lookup::lookup_structs::*,
    rootkit::rootkit::*,
    rootkit::rootkit_structs::*,
    cubes::cubes::*,
    cubes::cubes_structs::*,
    scanner::scanner::*,
    scanner::scanner_structs::*,
    osint::osint::*,
    osint::osint_structs::*,
};

use crate::shell::shell::init;

fn main() {
    init();
}

#[cfg(test)]
mod test;
