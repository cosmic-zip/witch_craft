#![allow(unused_variables)]
#![allow(dead_code)]

mod modules;

use crate::modules::core::core::*;

fn main() {
    println!("Hello, world!");
    lazy_exec("ls -lha".to_string(), true);
}

#[cfg(test)]
mod test;
