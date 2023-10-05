mod meow;
use crate::meow::meow::*;

use warp::{Filter, Rejection, Reply};
use std::io::{BufRead, BufReader};
use serde::Serialize;
use serde_json::*;
use std::fs::File;

#[derive(Serialize)]
struct JsonLine {
    content: String,
}

#[tokio::main]
async fn main() {

    let report = warp::path("report")
        .and(warp::get())
        .map(|| {
            // Path to report file
            let config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
            let path = format!("{}{}", config["REPORT_BASE_PATH"], config["REPORT_LOG"]);

            // Read lines from a file and convert them to JSON lines
            let file = File::open(path).expect("Failed to open file");
            let reader = BufReader::new(file);
            let json_lines: Vec<JsonLine> = reader
                .lines()
                .filter_map(|line| line.ok())
                .map(|line| JsonLine { content: line })
                .collect();

            // Serialize the JSON lines to a string
            let json_response = serde_json::to_string(&json_lines).expect("Failed to serialize");

            // Create a JSON response
            warp::reply::json(&json_response)
        });


    warp::serve(report)
        .run(([127, 0, 0, 1], 3030))
        .await;
}