use std::collections::HashMap;

pub const TONK: &str = "@@";
pub const SPLIT_II: &str = "--";
pub const SPLIT_I: &str = "-";


pub struct DataSet {
    pub name: String,
    pub meta: String,
}

impl DataSet {

    pub fn new() -> Self {
        DataSet {
            name: String::new(),
            meta: String::new(),
        }
    }

    pub fn from_str(a: &str, b:&str) -> Self {
        DataSet {
            name: a.to_string(),
            meta: b.to_string(),
        }
    }

}