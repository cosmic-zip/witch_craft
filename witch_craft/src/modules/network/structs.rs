use reqwest::blocking::Client;
use std::collections::HashMap;

struct Response {
    url: String,
    status: String,
    body: String,
}

struct Request {
    url: String,
    method: String,
    body: Option<HashMap<String, String>>,
}

impl Request {
    fn new() -> Self {
        Request {
            url: String::new(),
            method: String::new(),
            body: None,
        }
    }

    fn make(&self) -> Response {
        let client = Client::new();
        let res = match self.method.as_str() {
            "POST" => client.post(&self.url).json(&self.body).send().unwrap(),
            "GET" => client.get(&self.url).send().unwrap(),
            _ => panic!("Unsupported method"),
        };

        let status = res.status().to_string();
        let body = res.text().unwrap();

        Response {
            url: self.url.clone(),
            status,
            body,
        }
    }
}
