use reqwest::blocking::Client;
use std::collections::HashMap;

pub struct Response {
    pub url: String,
    pub status: String,
    pub body: String,
}

pub struct Request {
    pub url: String,
    pub method: String,
    pub body: Option<HashMap<String, String>>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            url: String::new(),
            method: String::new(),
            body: None,
        }
    }

    /// Make an request using a struct  Request and return and
    /// struct Response { ... }
    pub fn make(&self) -> Response {
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
