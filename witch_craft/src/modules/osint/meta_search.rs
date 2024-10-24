use super::data::META_LINKS;
use crate::core::core::*;
use crate::search_value;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use reqwest::blocking::Client;

fn exec_meta_search(data: (&str, &str, &str), keyword: &str) -> i32 {
    let client = Client::new();
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true) // Run the browser in headless mode
            .build()
            .unwrap(),
    )
    .unwrap();
    let tab = browser.new_tab().unwrap();
    let url = data.1.replace("@@keyword", &keyword);

    // Tab settings
    tab.set_default_timeout(std::time::Duration::from_secs(30));
    tab.navigate_to(&url).unwrap();
    tab.wait_until_navigated().unwrap();

    let content = tab.get_content().unwrap();

    match client.get(&url).send() {
        Ok(res) => {
            if res.status().as_u16() == 200 {
                if data.2.is_empty() {
                    raise(
                        &format!("Found! {} {} at {}", &keyword, data.0, &url),
                        "done",
                    );
                } else {
                    if content.contains(data.2) == false {
                        raise(
                            &format!("Found! {} {} at {}", &keyword, data.0, &url),
                            "done",
                        );
                    }
                }
            }

            return 0;
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return 1;
        }
    }
}

pub fn social_links(argsv: &[String]) -> i32 {
    let keyword = search_value("keyword", argsv);
    for data in META_LINKS {
        exec_meta_search(data, &keyword);
    }

    return 0;
}
