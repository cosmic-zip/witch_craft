use crate::core::core::*;
use crate::modules::osint::structs::{WebsiteEntry, WebsitesEntries};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use reqwest::blocking::Client;
use std::fs;

fn read_json_file(file_path: &str) -> WebsitesEntries {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let data: WebsitesEntries = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    data
}

pub fn exec_meta_search(data: WebsiteEntry, keyword: &str) {
    let url = &data.url.replace("@@keyword", &keyword);

    let client = Client::new();
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .window_size(Some((1920, 1080)))
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.new_tab().unwrap();
    tab.navigate_to(&url).unwrap();
    tab.wait_until_navigated().unwrap();
    let content = tab.get_content().unwrap();

    fn find_contents(data: WebsiteEntry, content: String) -> String {
        for item in data.detections {}
    }

    match client.get(url).send() {
        Ok(res) => {
            if res.status().as_u16() == 200 {
                // if content.contains(data.2) {}
            }
        }
        Err(err) => {
            raise(&format!("exec_meta_search :: {}", err.to_string()), "fail");
        }
    }
}

// pub fn social_links(argsv: &[String]) -> i32 {
//     let keyword = search_value("keyword", argsv);
//     let data = read_json_file_as_map(&get_witch_spells_path("osint/sites.json")).unwrap();

//     let websites = data.get("websites_entries").expect("Index not found");
//     let index_array = websites.as_array().expect("Index is not an array");

//     exec_meta_search(index_array, &keyword);

//     return 0;
// }
