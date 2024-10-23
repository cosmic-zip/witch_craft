use reqwest::blocking::Client;

use crate::core::core::*;

pub fn search_ans(argsv: &[String]) -> i32 {
    let mut ans_number = search_value("asn", argsv);
    ans_number = ip_to_number(&ans_number);

    if ans_number.len() <= 10 {
        let ans_ipv4: Vec<String> =
            read_file_to_lines("/var/witch_craft/witch_spells/osint/ans/ans.ipv4.csv");

        for line in ans_ipv4 {
            if line.as_str().contains(&ans_number) {
                raise(&format!("ANS Info:: {}", line), "done");
            }
        }
        return 0;
    }

    let ans_ipv6: Vec<String> =
        read_file_to_lines("/var/witch_craft/witch_spells/osint/ans/ans.ipv6.csv");

    for line in ans_ipv6 {
        if line.as_str().contains(&ans_number) {
            raise(&format!("ANS Info:: {}", line), "done");
        }
    }
    return 0;
}

pub fn search_geoloc(argsv: &[String]) -> i32 {
    let mut local = search_value("ip", argsv);
    local = ip_to_number(&local);

    if local.len() <= 10 {
        let ans_ipv4: Vec<String> =
            read_file_to_lines("/var/witch_craft/witch_spells/osint/geolocation/geodata.ipv4.csv");

        for line in ans_ipv4 {
            if line.as_str().contains(&local) {
                raise(&format!("GEODATA Info:: {}", line), "done");
            }
        }
        return 0;
    }

    let ans_ipv6: Vec<String> =
        read_file_to_lines("/var/witch_craft/witch_spells/osint/geolocation/geodata.ipv6.csv");

    for line in ans_ipv6 {
        if line.as_str().contains(&local) {
            raise(&format!("GEODATA Info:: {}", line), "done");
        }
    }
    return 0;
}

pub fn search_proxy(argsv: &[String]) -> i32 {
    let mut proxy = search_value("ip", argsv);
    proxy = ip_to_number(&proxy);

    if proxy.len() <= 10 {
        let ans_ipv4: Vec<String> =
            read_file_to_lines("/var/witch_craft/witch_spells/osint/proxy/proxy.ipv4.csv");

        for line in ans_ipv4 {
            if line.as_str().contains(&proxy) {
                raise(&format!("PROXY Info:: {}", line), "done");
            }
        }
        return 0;
    }

    let ans_ipv6: Vec<String> =
        read_file_to_lines("/var/witch_craft/witch_spells/osint/proxy/proxy.ipv6.csv");

    for line in ans_ipv6 {
        if line.as_str().contains(&proxy) {
            raise(&format!("PROXY Info:: {}", line), "done");
        }
    }
    return 0;
}

pub fn cinsscore(argsv: &[String]) -> i32 {
    let ip = search_value("ip", argsv);
    let file = read_file_to_lines("/var/witch_craft/witch_spells/osint/ci-badguys.txt");

    for line in file {
        if line.as_str().contains(&ip) {
            raise(&format!("IP found :: {}", line), "warning");
            return 255;
        }
    }

    raise("Nothing found, must be legit UwU", "message");
    return 0;
}

use headless_chrome::{Browser, LaunchOptionsBuilder};

pub fn social_links(argsv: &[String]) -> i32 {
    let keyword = search_value("keyword", argsv);
    // name - url - filter
    let social_links = [
        ("youtube", "https://www.youtube.com/@@@keyword", ""),
        ("gitlab", "https://gitlab.com/@@keyword", ""),
        ("github", "https://github.com/@@keyword", ""),
        ("bitbucket", "https://bitbucket.org/@@keyword", ""),
        (
            "facebook",
            "https://facebook.com/@@keyword",
            "This content isn't available right now",
        ),
        ("slideshare", "https://slideshare.net/@@keyword", ""),
        (
            "linkedin.corp",
            "https://linkedin.com/company/@@keyword",
            "",
        ),
        ("linkedin.user", "https://linkedin.com/in/@@keyword", ""),
        ("myspace", "https://myspace.com/@@keyword", ""),
        (
            "instagram",
            "https://instagram.com/@@keyword",
            "Sorry, this page isn't available.",
        ),
        (
            "medium",
            "https://medium.com/@@@keyword",
            "Out of nothing, something.",
        ),
        (
            "twitch",
            "https://twitch.tv/@@keyword",
            "Sorry. Unless you've got a time machine, that content is unavailable.",
        ),
        ("mastodon", "https://mastodon.social/@@@keyword", ""),
        (
            "bsky",
            "https://bsky.app/profile/@@keyword",
            "Error: handle must be a valid handle",
        ),
        ("reddit", "https://www.reddit.com/user/@@keyword", ""),
        (
            "twitter",
            "https://www.x.com/@@keyword",
            "This account doesn’t exist",
        ),
        (
            "xvideos",
            "https://www.xvideos.com/profiles/@@keyword",
            "THIS PROFILE DOESN'T EXIST !",
        ),
    ];

    for item in social_links {
        let client = Client::new();
        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                .headless(true) // Run the browser in headless mode
                .build()
                .unwrap(),
        )
        .unwrap();
        let tab = browser.new_tab().unwrap();
        let url = item.1.replace("@@keyword", &keyword);

        // Tab settings
        tab.set_default_timeout(std::time::Duration::from_secs(30));
        tab.navigate_to(&url).unwrap();
        tab.wait_until_navigated().unwrap();

        let content = tab.get_content().unwrap();

        match client.get(&url).send() {
            Ok(res) => {
                if res.status().as_u16() == 200 {
                    if item.2.is_empty() {
                        raise(
                            &format!("Found! {} {} at {}", &keyword, item.0, &url),
                            "done",
                        );
                    } else {
                        if content.contains(item.2) == false {
                            raise(
                                &format!("Found! {} {} at {}", &keyword, item.0, &url),
                                "done",
                            );
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
                return 1;
            }
        }
    }

    return 0;
}
