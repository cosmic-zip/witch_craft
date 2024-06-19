use crate::modules::core::core::*;
use crate::modules::core::data::*;
use crate::modules::core::structs::DataSet;

pub fn dns(argsv: Vec<String>) -> i32 {
    //Check if domain key exists
    if search_value("domain".to_string(), argsv.clone()) == "".to_string() {
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

        bob(set, argsv.clone());
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
        bob(extra, argsv.clone());
    }

    return 0;
}

pub fn plugin_file_compact(argsv: Vec<String>) -> i32 {
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

    let mut option = search_value("type".to_string(), argsv.clone());
    let mut command = String::new();

    if option == "decompress" {
        let file = search_value("file".to_string(), argsv.clone());
        for (ext, decomp, comp) in extensions {
            if file.ends_with(ext) {
                command = lazy_loop(decomp, argsv.clone());
            }
        }
    } else {
        let format = search_value("ext".to_string(), argsv.clone());
        for (ext, decomp, comp) in extensions {
            if ext == format {
                command = lazy_loop(comp, argsv.clone());
                println!("{}", command);
            }
        }
    }

    return 0;
}

pub fn logs_fallout(argsv: Vec<String>) -> i32 {
    let nukes = vec![
        DataSet::from_str("", "nuke.historic.unset_HISTFILE", "unset HISTFILE"),
        DataSet::from_str("", "nuke.historic.clear_history", "history -c"),
        DataSet::from_str(
            "",
            "nuke.historic.clear_sh_history",
            "cat /dev/null > ~/.sh_history",
        ),
        DataSet::from_str(
            "",
            "nuke.historic.clear_bash_history",
            "cat /dev/null > ~/.bash_history",
        ),
        DataSet::from_str("", "nuke.historic.clear_zsh_history_1", "history -c"),
        DataSet::from_str("", "nuke.historic.clear_zsh_history_2", "history -w"),
        DataSet::from_str(
            "",
            "nuke.historic.clear_zsh_history_3",
            "cat /dev/null > ~/.zsh_history",
        ),
        DataSet::from_str("", "nuke.historic.clear_zsh_history_4", "history -c"),
        DataSet::from_str("", "nuke.historic.clear_zsh_history_5", "history -w"),
        DataSet::from_str("", "nuke.logs.syslog", "cat /dev/null > /var/log/syslog"),
        DataSet::from_str(
            "",
            "nuke.logs.messages",
            "cat /dev/null > /var/log/messages",
        ),
        DataSet::from_str("", "nuke.logs.authlog", "cat /dev/null > /var/log/auth.log"),
        DataSet::from_str("", "nuke.logs.secure", "cat /dev/null > /var/log/secure"),
        DataSet::from_str("", "nuke.logs.kernlog", "cat /dev/null > /var/log/kern.log"),
        DataSet::from_str(
            "",
            "nuke.logs.applogs_1",
            "find /var/log -type f -name '*.log' -exec cat /dev/null > {} \\;",
        ),
        DataSet::from_str(
            "",
            "nuke.logs.auditlog",
            "cat /dev/null > /var/log/audit/audit.log",
        ),
        DataSet::from_str(
            "",
            "nuke.logs.loginlogs_wtmp",
            "cat /dev/null > /var/log/wtmp",
        ),
        DataSet::from_str(
            "",
            "nuke.logs.loginlogs_btmp",
            "cat /dev/null > /var/log/btmp",
        ),
        DataSet::from_str(
            "",
            "nuke.logs.loginlogs_lastlog",
            "cat /dev/null > /var/log/lastlog",
        ),
        DataSet::from_str("", "nuke.logs.alllogs_1", "cat /dev/null > /var/log/syslog"),
        DataSet::from_str(
            "",
            "nuke.logs.alllogs_2",
            "cat /dev/null > /var/log/messages",
        ),
        DataSet::from_str(
            "",
            "nuke.logs.alllogs_3",
            "cat /dev/null > /var/log/auth.log",
        ),
        DataSet::from_str("", "nuke.logs.alllogs_4", "cat /dev/null > /var/log/secure"),
        DataSet::from_str(
            "",
            "nuke.logs.alllogs_5",
            "cat /dev/null > /var/log/kern.log",
        ),
        DataSet::from_str(
            "",
            "nuke.logs.alllogs_6",
            "find /var/log -type f -name '*.log' -exec cat /dev/null > {} \\;",
        ),
        DataSet::from_str(
            "",
            "nuke.logs.alllogs_7",
            "cat /dev/null > /var/log/audit/audit.log",
        ),
        DataSet::from_str("", "nuke.logs.alllogs_8", "cat /dev/null > /var/log/wtmp"),
        DataSet::from_str("", "nuke.logs.alllogs_9", "cat /dev/null > /var/log/btmp"),
        DataSet::from_str(
            "",
            "nuke.logs.alllogs_10",
            "cat /dev/null > /var/log/lastlog",
        ),
    ];

    for nuke in nukes {
        bob(nuke, argsv.clone());
    }

    return 0;
}
