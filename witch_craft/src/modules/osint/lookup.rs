use crate::core::core::*;

pub fn search_ans(argsv: &[String]) -> i32 {
    let mut ans_number = search_value("asn", argsv);
    ans_number = ip_to_number(&ans_number);

    if ans_number.len() <= 10 {
        let ans_ipv4: Vec<String> =
            read_file_to_lines("/var/witch_craft/witch_spells/osint/ans/ans.ipv4.csv");

        for line in ans_ipv4 {
            if line.as_str().contains(&ans_number) {
                raise(&format!("ANS Info:: {}", line), 6);
            }
        }
        return 0;
    }

    let ans_ipv6: Vec<String> =
        read_file_to_lines("/var/witch_craft/witch_spells/osint/ans/ans.ipv6.csv");

    for line in ans_ipv6 {
        if line.as_str().contains(&ans_number) {
            raise(&format!("ANS Info:: {}", line), 6);
        }
    }
    return 0;
}

pub fn search_geoloc(argsv: &[String]) -> i32 {
    let mut local = search_value("local", argsv);
    local = ip_to_number(&local);

    if local.len() <= 10 {
        let ans_ipv4: Vec<String> =
            read_file_to_lines("/var/witch_craft/witch_spells/osint/geolocation/geodata.ipv4.csv");

        for line in ans_ipv4 {
            if line.as_str().contains(&local) {
                raise(&format!("GEODATA Info:: {}", line), 6);
            }
        }
        return 0;
    }

    let ans_ipv6: Vec<String> =
        read_file_to_lines("/var/witch_craft/witch_spells/osint/geolocation/geodata.ipv6.csv");

    for line in ans_ipv6 {
        if line.as_str().contains(&local) {
            raise(&format!("GEODATA Info:: {}", line), 6);
        }
    }
    return 0;
}

pub fn search_proxy(argsv: &[String]) -> i32 {
    let mut proxy = search_value("local", argsv);
    proxy = ip_to_number(&proxy);

    if proxy.len() <= 10 {
        let ans_ipv4: Vec<String> =
            read_file_to_lines("/var/witch_craft/witch_spells/osint/proxy/proxy.ipv4.csv");

        for line in ans_ipv4 {
            if line.as_str().contains(&proxy) {
                raise(&format!("PROXY Info:: {}", line), 6);
            }
        }
        return 0;
    }

    let ans_ipv6: Vec<String> =
        read_file_to_lines("/var/witch_craft/witch_spells/osint/proxy/proxy.ipv6.csv");

    for line in ans_ipv6 {
        if line.as_str().contains(&proxy) {
            raise(&format!("PROXY Info:: {}", line), 6);
        }
    }
    return 0;
}
