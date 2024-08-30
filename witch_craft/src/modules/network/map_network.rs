use crate::core::core::*;
use crate::core::structs::DataSet;

pub fn map_dns(argsv: &[String]) -> i32 {
    if search_value("domain", argsv).is_empty() {
        raise("Domain name not found, quit!", 4);
        return 42;
    }

    let record_types = vec![
        "A", "AAAA", "CNAME", "MX", "NS", "TXT", "SOA", "SRV", "PTR", "DNSKEY",
    ];

    for record_type in &record_types {
        let meta = format!("dig @@domain {} +short", record_type);
        let name = format!("dns.{}", record_type.to_lowercase());
        let set = DataSet::from_str("", &name, &meta);

        flawless_exec(set, argsv);
    }

    let extras = vec![
        DataSet::from_str("", "extras.whois", "whois @@domain"),
        DataSet::from_str(
            "",
            "extras.robots.txt",
            "cdomain -sS -L https://@@domain/robots.txt",
        ),
        DataSet::from_str(
            "",
            "extras.sitemap",
            "cdomain -sS -L https://@@domain/sitemap.xml",
        ),
    ];
    for extra in extras {
        flawless_exec(extra, argsv);
    }

    0
}
