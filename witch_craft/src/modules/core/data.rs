use crate::modules::core::structs::DataSet;

pub const TONK: &str = "@@";
pub const SPLIT_II: &str = "--";
pub const SPLIT_I: &str = "-";

pub fn data() -> Vec<DataSet> {
    let nuke = vec![
        DataSet::from_str("", "nuke.hd", "shred -vzn 7 @@device"),
        DataSet::from_str("", "nuke.file", "shred -u -z -n 7 @@file"),
    ];

    let view_sys_log = vec![
        DataSet::from_str("", "view.syslog", "tail -n 100 /var/log/syslog"),
        DataSet::from_str("", "view.messages", "tail -n 100 /var/log/messages"),
        DataSet::from_str("", "view.authlog", "tail -n 100 /var/log/auth.log"),
        DataSet::from_str("", "view.secure", "tail -n 100 /var/log/secure"),
        DataSet::from_str("", "view.kernlog", "tail -n 100 /var/log/kern.log"),
        DataSet::from_str("", 
            "view.applogs",
            "find /var/log -type f -name \"*.log\" -exec tail -n 100 {} \\;",
        ),
        DataSet::from_str("", "view.auditlog", "tail -n 100 /var/log/audit/audit.log"),
        DataSet::from_str("", "view.wtmp", "last -n 100"),
        DataSet::from_str("", "view.btmp", "lastb -n 100"),
        DataSet::from_str("", "view.lastlog", "tail -n 100 /var/log/lastlog"),
    ];

    let xxd = vec![
        DataSet::from_str("", "file.hex", "xxd @@file"),
        DataSet::from_str("", "file.bin", "xxd -b @@file"),
        DataSet::from_str("", "file.dec", "xxd -d @@file"),
        DataSet::from_str("", "file.dump", "xxd -ps @@file"),
        DataSet::from_str("", "file.list", "xxd -i @@file"),
        DataSet::from_str("", "file.link", "ldd -v @@file"),
        DataSet::from_str("", "file", "file @@file"),
    ];

    let metadata = vec![DataSet::from_str("", "meta.img", "exiftool @@file")];

    let dns_brute_force = vec![
        DataSet::from_str("", "dns.attack.subs", "dirb @@domain -w @@wordlist -o @@output"),
        DataSet::from_str("", 
            "dns.attack.file",
            "dnsenum --enum @@domain -t 15 --threads 4 -f @@wordlist -o @@output",
        ),
    ];

    let iptables = vec![
        DataSet::from_str("", 
            "web.donwload", 
            "wget --recursive --no-clobber --page-requisites --html-extension --convert-links --restrict-file-names=windows --random-wait --wait=@@wait --limit-rate=200k --tries=inf --domains example.com --no-parent @@url"
        ),
        DataSet::from_str("", 
            "firewall.home",
            "iptables -F && iptables -A INPUT -p tcp --dport 8000:65535 -j DROP && iptables -A OUTPUT -p tcp --dport 8000:65535 -j DROP && ip6tables -F && ip6tables -A INPUT -p tcp --dport 8000:65535 -j DROP && ip6tables -A OUTPUT -p tcp --dport 8000:65535 -j DROP",
        ),
        DataSet::from_str("", 
            "firewall.work",
            "iptables -F && iptables -A INPUT -j DROP && ip6tables -F && ip6tables -A INPUT -j DROP",
        ),
        DataSet::from_str("", 
            "firewall.public",
            "iptables -F && iptables -A INPUT -j DROP && iptables -A OUTPUT -p tcp --dport 8000:65535 -j DROP && ip6tables -F && ip6tables -A INPUT -j DROP && ip6tables -A OUTPUT -p tcp --dport 8000:65535 -j DROP",
        ),
        DataSet::from_str("", 
            "firewall.paranoid",
            "iptables -P INPUT DROP && iptables -P OUTPUT DROP && ip6tables -P INPUT DROP && ip6tables -P OUTPUT DROP",
        )
    ];

    let nmap = vec![
        DataSet::from_str("", 
            "map.tcp",
            "nmap -sS -T2 -p- -A -O -D RND:10 -f --data-length 32 -Pn @@target",
        ),
        DataSet::from_str("", 
            "map.udp",
            "nmap -sU -T2 -p- -A -O -D RND:10 -f --data-length 32 -Pn @@target",
        ),
        DataSet::from_str("", 
            "map.ping",
            "nmap -sP -T2 -D RND:10 -f --data-length 32 @@target",
        ),
        DataSet::from_str("", 
            "map.tcp_udp",
            "nmap -sS -sU -T2 -p- -A -O -D RND:10 -f --data-length 32 -Pn @@target",
        ),
    ];

    let general = vec![
        DataSet::from_str("", "map.test", "echo @@some"),
        DataSet::from_str("", "map.local", "ss -tupanr"),
        DataSet::from_str("", "list.processes.using.file", "lsof @@file_path"),
    ];

    return [
        nuke,
        view_sys_log,
        xxd,
        metadata,
        dns_brute_force,
        iptables,
        nmap,
        general,
    ]
    .concat();
}
