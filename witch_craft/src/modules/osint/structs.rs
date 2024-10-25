#[derive(Debug, Clone)]
pub struct MetaSearch {
    pub status: i32,
    pub url: String,
    pub flag: String,
    pub body: String,
}

impl MetaSearch {
    pub fn new(status: i32, url: String, flag: String, body: String) -> Self {
        MetaSearch {
            status: status,
            url: url,
            flag: flag,
            body: body,
        }
    }
}
