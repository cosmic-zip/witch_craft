mod modules;

use crate::modules::shell::shell::shell;

#[allow(dead_code)]
fn main() {
    shell();
}

#[cfg(test)]
mod test;
