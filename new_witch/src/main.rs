#![allow(unused_variables)]
#![allow(dead_code)]

mod modules;

use crate::modules::core::core::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test;
