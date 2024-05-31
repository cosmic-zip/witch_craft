use crate::modules::core::core::*;
use crate::modules::core::data::*;


pub fn dns(argsv: Vec<String>) -> i32 {
    let record_types = vec!["A", "AAAA", "CNAME", "MX", "NS", "TXT", "SOA", "SRV", "PTR", "DNSKEY"];

    for record_type in &record_types {
        let meta = format!("dig @@domain {} +short", record_type);
        let name = format!("dns.{}", record_type.to_lowercase());
        let set = DataSet::from_str(&name, &meta);

        println!("🟣 {}", set.name);
        let cmd = lazy_loop(&set.meta, argsv.clone());
        let out = lazy_exec(cmd, true);

        if out != 0 {
            return out;
        }

    }

    return 0;
}