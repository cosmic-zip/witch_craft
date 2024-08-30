use crate::core::consts::*;
use crate::core::structs::DataSet;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use super::core::raise;

#[derive(Debug, Deserialize)]
struct JsonEntry {
    description: String,
    name: String,
    command: String,
}

#[derive(Debug, Deserialize)]
struct JsonData {
    general: Vec<JsonEntry>,
}

fn read_dataset() -> Option<Vec<DataSet>> {
    let mut file = match File::open(DBPATH) {
        Ok(file) => file,
        Err(err) => {
            raise(&err.to_string(), 4);
            return None;
        }
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return None;
    }

    let json_data: JsonData = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(err) => {
            raise(&err.to_string(), 4);
            return None;
        }
    };

    let dataset: Vec<DataSet> = json_data
        .general
        .iter()
        .map(|entry| DataSet::from_str(&entry.description, &entry.name, &entry.command))
        .collect();

    Some(dataset)
}

pub fn data() -> Vec<DataSet> {
    match read_dataset() {
        Some(data) => data,
        None => {
            raise("Dataset are empty", 2);
            return vec![];
        }
    }
}