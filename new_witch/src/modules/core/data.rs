pub const TONK: &str = "@@";
pub const SPLIT_II: &str = "--";
pub const SPLIT_I: &str = "-";

pub struct DataSet {
    pub name: String,
    pub meta: String,
}

impl DataSet {
    pub fn new() -> Self {
        DataSet {
            name: String::new(),
            meta: String::new(),
        }
    }

    pub fn from_str(a: &str, b: &str) -> Self {
        DataSet {
            name: a.to_string(),
            meta: b.to_string(),
        }
    }
}

pub fn data() -> Vec<DataSet> {
    return vec![
        DataSet::from_str("map.test", "echo @@some"),
        DataSet::from_str("map.local", "ss -tupanr"),
        DataSet::from_str("list.processes.using.file", "lsof @@file_path"),
        DataSet::from_str("map.tcp", "nmap -sS -T2 -p- -A -O -D RND:10 -f --data-length 32 -Pn @@target"),
        DataSet::from_str("map.udp", "nmap -sU -T2 -p- -A -O -D RND:10 -f --data-length 32 -Pn @@target"),
        DataSet::from_str("map.ping", "nmap -sP -T2 -D RND:10 -f --data-length 32 @@target"),
        DataSet::from_str("map.tcp_udp", "nmap -sS -sU -T2 -p- -A -O -D RND:10 -f --data-length 32 -Pn @@target"),
        DataSet::from_str(
            "web.donwload", 
            "wget --recursive --no-clobber --page-requisites --html-extension --convert-links --restrict-file-names=windows --random-wait --wait=@@wait --limit-rate=200k --tries=inf --domains example.com --no-parent @@url"
        ),
        DataSet::from_str(
            "firewall.home",
            "iptables -F && iptables -A INPUT -p tcp --dport 8000:65535 -j DROP && iptables -A OUTPUT -p tcp --dport 8000:65535 -j DROP && ip6tables -F && ip6tables -A INPUT -p tcp --dport 8000:65535 -j DROP && ip6tables -A OUTPUT -p tcp --dport 8000:65535 -j DROP",
        ),
        DataSet::from_str(
            "firewall.work",
            "iptables -F && iptables -A INPUT -j DROP && ip6tables -F && ip6tables -A INPUT -j DROP",
        ),
        DataSet::from_str(
            "firewall.public",
            "iptables -F && iptables -A INPUT -j DROP && iptables -A OUTPUT -p tcp --dport 8000:65535 -j DROP && ip6tables -F && ip6tables -A INPUT -j DROP && ip6tables -A OUTPUT -p tcp --dport 8000:65535 -j DROP",
        ),
        DataSet::from_str(
            "firewall.paranoid",
            "iptables -P INPUT DROP && iptables -P OUTPUT DROP && ip6tables -P INPUT DROP && ip6tables -P OUTPUT DROP",
        )
    ];
}
