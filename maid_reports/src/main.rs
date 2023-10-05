mod meow;
mod api;

use crate::api::api::*;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a filter that matches a GET request to the /jsonl endpoint.
    let jsonl_route = warp::path!("jsonl")
        .and(warp::get())
        .map(|| {
            // Read the JSONL file and return it as a response.
            let file_content = read_jsonl_file();
            warp::reply::with_header(file_content, warp::http::header::CONTENT_TYPE, "application/json")
        });

    // Start the Warp server.
    warp::serve(jsonl_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}