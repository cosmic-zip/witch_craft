extern crate rusqlite;
use rusqlite::Error;
use rusqlite::Connection;


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


pub fn select_report_from_db(conn: &Connection, from: String, size: i32) -> Result<Vec<Vec<String>>, Error> {
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
                format!("{}", row.get::<usize, i32>(0).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(1).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(2).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(3).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(4).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(5).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(6).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(7).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(8).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(9).expect("Error at src.core.selec_report_db")),
                is_string_mem_empty(row.get::<usize, String>(10).expect("Error at src.core.selec_report_db")),
                is_bool_mem_empty(row.get::<usize, i32>(11).expect("Error at src.core.selec_report_db")),
            ]
        )
    })?;


    for row in rows {
        match row {
            Ok(data) => formated_rows.push(data),
            Err(_) => todo!(),
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

#[tauri::command]
pub fn dummy() -> String {
    format!("Hello")
}
