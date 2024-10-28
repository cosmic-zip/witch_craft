#[derive(Debug, Clone)]
pub struct DataSet {
    pub docs: String,
    pub name: String,
    pub meta: String,
}

impl DataSet {
    pub fn _new() -> Self {
        DataSet {
            docs: String::new(),
            name: String::new(),
            meta: String::new(),
        }
    }

    pub fn from_str(docs: &str, name: &str, meta: &str) -> Self {
        DataSet {
            docs: docs.to_string(),
            name: name.to_string(),
            meta: meta.to_string(),
        }
    }
}
