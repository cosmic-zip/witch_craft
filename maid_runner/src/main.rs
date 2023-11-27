#![allow(unused_imports)]
#![allow(unused_assignments)]

mod core;
mod meow;
mod modules;
mod shell;

use crate::core::utils::*;
use crate::meow::meow::*;
use crate::modules::{
    curl::curl::*,
    curl::curl_structs::*,
    // botnet::botnet::*,
    // botnet::botnet_structs::*,
    lookup::lookup::*,
    lookup::lookup_structs::*,
    // attack::attack::*,
    // attack::attack_structs::*,
    maid_av::maid_av::*,
    maid_av::maid_av_structs::*,
    // rootkit::rootkit::*,
    // rootkit::rootkit_structs::*,
    scanner::scanner::*,
    scanner::scanner_structs::*,
    firewall::firewall::*,
    firewall::firewall_structs::*,
};

use crate::shell::shell::init;

fn main() {
    init();
}

#[cfg(test)]
mod test;
