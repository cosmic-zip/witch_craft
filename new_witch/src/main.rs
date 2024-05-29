#![allow(unused_variables)]
#![allow(dead_code)]

mod modules;

use crate::modules::core::core::*;

fn main() {
    println!("Works!");
}

#[cfg(test)]
mod test;
