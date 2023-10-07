mod api;
mod meow;

use crate::api::api::*;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a filter that matches a GET request to the /jsonl endpoint.
     // Replace "your_jsonl_file.jsonl" with the path to your JSONL file.
    let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let file_path = format!("{}{}", config["REPORT_BASE_PATH"], config["REPORT_LOG"]);
 
    let jsonl_route = warp::path!("jsonl").and(warp::get()).map(|| {
        // Read the JSONL file and return it as a response.
        let file_content = read_file(file_path);
        warp::reply::with_header(
            file_content,
            warp::http::header::CONTENT_TYPE,
            "application/json",
        )
    });


    let index_page = config["INDEX"].to_string();
    let index = warp::path!("/").and(warp::get()).map(|| {
        let file_content = read_file(index_page);
        warp::reply::with_header(
            file_content,
            warp::http::header::CONTENT_TYPE,
            "text/html",
        )
    });

    // Start the Warp server.
    warp::serve(jsonl_route, index).run(([127, 0, 0, 1], 8000)).await;
}
