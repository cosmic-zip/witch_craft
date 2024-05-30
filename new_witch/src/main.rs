mod modules;

use crate::modules::core::core::*;
use crate::modules::core::data::*;

fn main() {
    let data = vec![
        DataSet::from_str("map.local", "ss -tupanr"),
        DataSet::from_str("map.external", "echo @@target"),
        DataSet::from_str("map.udp", "nmap -Ss"),
        DataSet::from_str(
            "web.donwload", 
            "wget --recursive --no-clobber --page-requisites --html-extension --convert-links --restrict-file-names=windows --random-wait --wait=@@wait --limit-rate=200k --tries=inf --domains example.com --no-parent @@url"
        ),
        DataSet::from_str("", ""),
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
        ),
        
    ];

    let argsv = readargs();
    for set in data {
        if set.name == argsv[1] {
            let cmd = lazy_loop(&set.meta, argsv.clone());
            lazy_exec(cmd, true);
        }
    }
}

#[cfg(test)]
mod test;
