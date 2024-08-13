use std::collections::HashMap;
use std::iter::repeat;

use crate::modules::core::core::*;
use crate::modules::core::structs::DataSet;
use crate::modules::network::structs::*;

pub fn map_dns(argsv: &[String]) -> i32 {
    //Check if domain key exists
    if search_value("domain", &argsv).is_empty() {
        raise("Domain name not found, quit!", 4);
        return 1;
    }

    let record_types = vec![
        "A", "AAAA", "CNAME", "MX", "NS", "TXT", "SOA", "SRV", "PTR", "DNSKEY",
    ];

    for record_type in &record_types {
        let meta = format!("dig @@domain {} +short", record_type);
        let name = format!("dns.{}", record_type.to_lowercase());
        let set = DataSet::from_str("", &name, &meta);

        flawless_exec(set, &argsv);
    }

    // Perform extra scans
    let extras = vec![
        DataSet::from_str("", "extras.whois", "whois @@domain"),
        DataSet::from_str(
            "",
            "extras.robots.txt",
            "curl -sS -L https://@@domain/robots.txt",
        ),
        DataSet::from_str(
            "",
            "extras.sitemap",
            "curl -sS -L https://@@domain/sitemap.xml",
        ),
    ];
    for extra in extras {
        flawless_exec(extra, &argsv);
    }

    return 0;
}

pub fn dos_simple_get_span(argsv: &[String]) -> i32 {
    let mut req = Request::new();
    req.url = search_value("url", &argsv);
    req.method = "GET".to_string();

    match search_key("repeat", argsv).parse::<i32>() {
        Ok(times) => {
            for _ in 0..times {
                req.make();
            }
            return 0;
        }
        Err(_) => 255,
    }
}

pub fn seach_number_value(key: &str, argsv: &[String]) -> i32 {
    match search_value(key, &argsv).parse::<i32>() {
        Ok(number) => number,
        Err(_) => -999999,
    }
}

pub fn dos_long_auth_span(argsv: &[String]) -> i32 {
    let size = seach_number_value("size", &argsv);
    let seed = "3l34_=3k4vç~4vu,,20-v";
    let mut req = Request::new();
    req.url = search_value("url", &argsv);
    req.method = "GET".to_string();
    req.body = Some(HashMap::from([
        ("user", seed),
        ("pass", seed),
        ("username", seed),
        ("password", seed),
        ("token", seed),
        ("auth", seed),
    ]));

    match search_key("repeat", &argsv).parse::<i32>() {
        Ok(times) => {
            for _ in 0..times {
                req.make();
            }
            return 0;
        }
        Err(_) => 255,
    }
}
