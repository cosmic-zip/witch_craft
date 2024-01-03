extern crate rusqlite;
use rusqlite::Error;
use rusqlite::Connection;
use crate::meow::meow::read_meow;
use std::collections::HashMap;



pub fn is_string_mem_empty(mem_loc: String) -> String {
    if mem_loc.len() == 0 || mem_loc == "[]".to_string() {
        "none".to_string()
    } else {
        mem_loc
    }
}

pub fn is_bool_mem_empty(mem_bool: i32) -> String {
    if mem_bool == 0 {
        "false".to_string()
    } else {
        "true".to_string()
    }
} 

pub fn return_error_vec_hash_map() -> Vec<HashMap<String, String>> {

    return vec![
        HashMap::from([
            ("error".to_string(), "select.all".to_string())
        ])
    ]

}

pub fn select_report_from_db(conn: &Connection, from: String, size: i32) -> Result<Vec<HashMap<String, String>>, Error> {
    let mut result;

    if from == "all" {
        result = match conn.prepare(&format!(
            "select * FROM process_results ORDER BY id DESC LIMIT {}", size,
        )) {
            Ok(value) => value,
            Err(_) => {
                return Ok(return_error_vec_hash_map());
            }
        }
    } else {
        result = match conn.prepare(
            &format!("select * FROM process_results WHERE source_from=\"{}\" ORDER BY id DESC LIMIT {}; ", from, size)
        ) {
            Ok(value) => value,
            Err(_) => {
                return Ok(return_error_vec_hash_map());
            }
        }
    }

    let mut formatted_rows: Vec<HashMap<String, String>> = Vec::new();

    let rows = result.query_map([], |row| {
        // Create a ProcessResult struct and populate its fields
        Ok(
            HashMap::from([
                ("id".to_string(), format!("{}", row.get::<usize, i32>(0).expect("ERROR.map.report"))),
                ("session".to_string(), is_string_mem_empty(row.get::<usize, String>(1).expect("ERROR.map.report"))),
                ("session_description".to_string(), is_string_mem_empty(row.get::<usize, String>(2).expect("ERROR.map.report"))),
                ("source_from".to_string(), is_string_mem_empty(row.get::<usize, String>(3).expect("ERROR.map.report"))),
                ("source_command".to_string(), is_string_mem_empty(row.get::<usize, String>(4).expect("ERROR.map.report"))),
                ("source_detail".to_string(), is_string_mem_empty(row.get::<usize, String>(5).expect("ERROR.map.report"))),
                ("source_description".to_string(), is_string_mem_empty(row.get::<usize, String>(6).expect("ERROR.map.report"))),
                ("timestemp".to_string(), is_string_mem_empty(row.get::<usize, String>(7).expect("ERROR.map.report"))),
                ("returned_status".to_string(), is_string_mem_empty(row.get::<usize, String>(8).expect("ERROR.map.report"))),
                ("formatted_stdout".to_string(), is_string_mem_empty(row.get::<usize, String>(9).expect("ERROR.map.report"))),
                ("formatted_stderr".to_string(), is_string_mem_empty(row.get::<usize, String>(10).expect("ERROR.map.report"))),
                ("debug".to_string(), is_bool_mem_empty(row.get::<usize, i32>(11).expect("ERROR.map.report"))),
            ])
        )
    })?;


    for row in rows {
        match row {
            Ok(data) => formatted_rows.push(data),
            Err(_) => panic!("Where"),
        }
        
    }

    Ok(formatted_rows)
}

#[tauri::command]
pub fn select_report(from: String, size: i32) -> Vec<HashMap<String, String>> {

    let report_config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let report = format!(
        "{}{}",
        report_config["REPORT_BASE_PATH"], report_config["REPORT_LOG"]
    );


    let connection = Connection::open(report).expect("fail");

    match select_report_from_db(&connection, from, size) {
        Ok(process_results) => process_results,
        Err(_) => return_error_vec_hash_map(),
    } 
    
}

#[tauri::command]
pub fn dummy() -> String {
    format!("Hello")
}
