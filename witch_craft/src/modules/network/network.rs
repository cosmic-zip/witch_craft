use crate::core::types::Closure;
use crate::modules::network::ddos::*;
use crate::modules::network::map_network::*;
use crate::modules::network::server::evil_server;

pub fn api() -> Closure {
    vec![
        ("dos.longpw", dos_long_auth_span),
        ("dos.spam", dos_simple_get_span),
        ("map.dns", map_dns),
        ("server.eviltwin", evil_server),
    ]
}
