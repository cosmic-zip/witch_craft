use crate::modules::core::core::*;
use crate::modules::core::structs::DataSet;
use crate::modules::network::structs::*;
use std::collections::HashMap;

/// Revice --domain
pub fn map_dns(argsv: &[String]) -> i32 {
    //Check if domain key exists
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

    // Perform extra scans
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

    return 0;
}

/// Need:
/// --domain domain
/// --times int
pub fn dos_simple_get_span(argsv: &[String]) -> i32 {
    let mut req = Request::new();
    req.url = search_value("domain", argsv);
    req.method = "GET".to_string();

    let times = seach_number_value("times", argsv);

    for _i in 0..times {
        let out = req.make();
        println!("{} - {}", out.url, out.status);
    }
    return 0;
}

/// Need:
/// --size int Size of string attak
/// --domain domain  Target domain
/// --times int
pub fn dos_long_auth_span(argsv: &[String]) -> i32 {
    let size = seach_number_value("size", argsv);
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
    return 0;
}

/// Compress and Decompress files
pub fn file_compact(argsv: Vec<String>) -> i32 {
    let extensions = vec![
        ("7z", "7z x @@file", "7z a @@folder"),
        ("arj", "arj x @@file", "arj a @@folder"),
        ("bz2", "bzip2 -d @@file", "bzip2 @@folder"),
        ("cab", "cabextract @@file", "cabarc n @@folder"),
        ("deb", "dpkg -x @@file .", "dpkg -b @@folder"),
        (
            "dmg",
            "hdiutil attach @@file -mountpoint /tmp/mount",
            "hdiutil create -srcfolder @@folder",
        ),
        ("gz", "gunzip @@file", "gzip @@folder"),
        ("iso", "isoinfo -i @@file -x /", "genisoimage -o @@folder"),
        ("lz", "lzip -d @@file", "lzip @@folder"),
        ("lzma", "lzma -d @@file", "lzma @@folder"),
        ("lzo", "lzop -d @@file", "lzop @@folder"),
        ("lzop", "lzop -d @@file", "lzop @@folder"),
        ("rar", "unrar x @@file", "rar a @@folder"),
        (
            "rpm",
            "rpm2cpio @@file | cpio -idmv",
            "rpmbuild -ba @@folder",
        ),
        ("tar", "tar -xvf @@file", "tar -cvf @@folder"),
        ("tar.bz2", "tar -xvjf @@file", "tar -cvjf @@folder"),
        ("tar.gz", "tar -xvzf @@file", "tar -cvzf @@folder"),
        ("tar.xz", "tar -xvJf @@file", "tar -cvJf @@folder"),
        ("xz", "unxz @@file", "xz @@folder"),
        ("Z", "uncompress @@file", "compress @@folder"),
        ("zip", "unzip @@file", "zip -r @@folder"),
    ];

    let option = search_value("type", &argsv);
    let mut command = String::new();

    if option == "decompress" {
        let file = search_value("file", &argsv);
        for (ext, decomp, _comp) in extensions {
            if file.ends_with(ext) {
                command = lazy_parser(decomp, &argsv);
            }
        }
    } else {
        let format = search_value("ext", &argsv);
        for (ext, _decomp, comp) in extensions {
            if ext == format {
                command = lazy_parser(comp, &argsv);
                println!("{}", command);
            }
        }
    }
    lazy_exec(command)
}
