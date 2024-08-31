use crate::{core::types::Closure, modules::blackcat::backend::blackcat_av};

pub fn api() -> Closure {
    vec![("blackcat.av", blackcat_av)]
}
