use crate::{
    core::types::Closure,
    modules::blackcat::backend::{blackcat_av, blackcat_chkrootkit_bind},
};

pub fn api() -> Closure {
    vec![
        ("blackcat.av", blackcat_av),
        ("blackcat.rootkit", blackcat_chkrootkit_bind),
    ]
}
