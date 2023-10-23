extern crate rusqlite;
use rusqlite::Error;
use rusqlite::Connection;
use serde_json::json;



pub fn  select_report_from_db(conn: &Connection, from: String, size: i32) -> Result<Vec<Vec<String>>, Error> {
    let mut result;

    if from == "all" {
        result = conn.prepare(&format!(
            "SELECT * FROM process_results ORDER BY id DESC LIMIT {}", size,
        )).expect("Query Failed");
    } else {
        result = conn.prepare(
            &format!("SELECT * FROM process_results WHERE source_from=\"{}\" ORDER BY id DESC LIMIT {}; ", from, size
        )).expect("Query Failed");
    }

    let mut formated_rows: Vec<Vec<String>> = Vec::new();

    let rows = result.query_map([], |row| {
        // Create a ProcessResult struct and populate its fields
        Ok(
            vec![
                format!("{}", row.get::<usize, i32>(0).expect("fodase")),
                row.get::<usize, String>(1).expect("fodase"),
                row.get::<usize, String>(2).expect("fodase"),
                row.get::<usize, String>(3).expect("fodase"),
                row.get::<usize, String>(4).expect("fodase"),
                row.get::<usize, String>(5).expect("fodase"),
                row.get::<usize, String>(6).expect("fodase"),
                row.get::<usize, String>(7).expect("fodase"),
                row.get::<usize, String>(8).expect("fodase"),
                row.get::<usize, String>(9).expect("fodase"),
                row.get::<usize, String>(10).expect("fodase"),
                format!("{}", row.get::<usize, i32>(11).expect("fodase")),
            ]
        )
    })?;


    for row in rows {
        match row {
            Ok(data) => formated_rows.push(data),
            Err(_) => panic!("[PANIC] :: Unknow error"),
        }
        
    }

    Ok(formated_rows)
}

#[tauri::command]
pub fn select_report(from: String, size: i32) -> Vec<Vec<String>> {
    let connection = Connection::open("/var/maid/maid_lists/report/archive.db").expect("fail");

    match select_report_from_db(&connection, from, size) {
        Ok(process_results) => process_results,
        Err(_) => vec![],
    } 
    
}