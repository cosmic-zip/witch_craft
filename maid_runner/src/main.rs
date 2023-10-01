#![allow(unused_imports)]

mod core;
mod meow;
mod modules;
mod shell;

use crate::core::utils::*;
use crate::modules::{
    curl::curl::*,
    curl::curl_structs::*,
    // attack::attack::*,
    // attack::attack_structs::*,
    maid_av::maid_av::*,
    maid_av::maid_av_structs::*,
    // botnet::botnet::*,
    // botnet::botnet_structs::*,
    lookup::lookup::*,
    lookup::lookup_structs::*,
    // post_attack::post_attack::*,
    // post_attack::post_attack_structs::*,
    scanner::scanner::*,
    scanner::scanner_structs::*,
};
use crate::shell::shell::init;

fn main() {
    init();
}

#[cfg(test)]
mod test;
