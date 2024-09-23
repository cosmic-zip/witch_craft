use crate::core::types::Closure;
use crate::modules::osint::lookup::*;

pub fn api() -> Closure {
    vec![
        ("search.ans", search_ans),
        ("search.geoloc", search_geoloc),
        ("search.proxy", search_proxy),
    ]
}
