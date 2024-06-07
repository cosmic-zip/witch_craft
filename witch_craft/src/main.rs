mod modules;

use crate::modules::shell::shell::shell;

fn main() {
    shell();
}

#[cfg(test)]
mod test;
