use super::{data::META_LINKS, structs::MetaSearch};
use crate::core::core::*;
use crate::search_value;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::blocking::Client;

pub fn exec_meta_search(data: (&str, &str, &str), keyword: &str) -> MetaSearch {
    let timeout = std::time::Duration::from_secs(60);
    let url = data.1.replace("@@keyword", &keyword);
    let client = Client::new();

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .idle_browser_timeout(timeout)
            .window_size(Some((1920, 1080)))
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.new_tab().unwrap();
    tab.navigate_to(&url).unwrap();
    tab.wait_until_navigated().unwrap();
    let content = tab.get_content().unwrap();

    match client.get(&url).send() {
        Ok(res) => {
            if res.status().as_u16() == 200 {
                if content.contains(data.2) {
                    return MetaSearch::new(200, url, data.2.to_string(), content);
                }

                raise(
                    &format!("Found! {} {} at {}", &keyword, data.0, &url),
                    "done",
                );
                return MetaSearch::new(200, url, "".to_string(), content);
            }

            return MetaSearch::new(0, url, "".to_string(), content);
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return MetaSearch::new(1, url, "".to_string(), err.to_string());
        }
    }
}

pub fn social_links(argsv: &[String]) -> i32 {
    let keyword = search_value("keyword", argsv);

    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    raise(
        "Some links may have false positives, especially on websites with client-side rendering.",
        "warning",
    );

    META_LINKS.par_iter().for_each(|&data| {
        exec_meta_search(data, &keyword);
    });

    raise("All tasks completed!", "done");
    return 0;
}
