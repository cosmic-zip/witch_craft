#![allow(unused_mut)]

use crate::core::utils::*;
use crate::meow::meow::read_meow;
use std::collections::HashMap;

pub fn entropy_calculos(actual: f64, past: f64) -> f64 {
    let result = (actual - past) / past * 100.0;

    if result.is_nan() {
        return 3440.3440;
    }
    if result == f64::INFINITY {
        return 100.0;
    }

    result
}

pub fn entropy_filter(line: String) -> HashMap<&'static str, f64> {
    const GENERAL: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;?@[\\]^_`{{|}}~\n\r\x0b\x0c";
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "!\"#$%&'()*+,-./:;?@[\\]^_`{{|}}~\n\r\x0b\x0c";

    let mut entropy_result = HashMap::new();

    // Possitive entropy refer to valid ascii charadcters in they class

    let mut count_entropy_general_possitive: f64 = 0.0;
    let mut count_entropy_letters_possitive: f64 = 0.0;
    let mut count_entropy_numbers_possitive: f64 = 0.0;
    let mut count_entropy_symbols_possitive: f64 = 0.0;

    let mut chars_lenth: f64 = line.len() as f64;

    if chars_lenth.is_nan() || chars_lenth == 0.0 {
        entropy_result.insert("entropy_general_possitive", 0.0);
        entropy_result.insert("entropy_letters_possitive", 0.0);
        entropy_result.insert("entropy_numbers_possitive", 0.0);
        entropy_result.insert("entropy_symbols_possitive", 0.0);
        entropy_result.insert("entropy_general_negative", 0.0);
        entropy_result.insert("entropy_letters_negative", 0.0);
        entropy_result.insert("entropy_numbers_negative", 0.0);
        entropy_result.insert("entropy_symbols_negative", 0.0);
        entropy_result.insert("entropy_general_percent_of_bin", 0.0);
        entropy_result.insert("entropy_letters_percent_of_bin", 0.0);
        entropy_result.insert("entropy_numbers_percent_of_bin", 0.0);
        entropy_result.insert("entropy_symbols_percent_of_bin", 0.0);

        return entropy_result;
    }

    for character in line.chars() {
        println!("{}", character);

        if GENERAL.contains(character) {
            count_entropy_general_possitive = count_entropy_general_possitive + 1.0;
        }

        if LETTERS.contains(character) {
            count_entropy_letters_possitive = count_entropy_letters_possitive + 1.0;
        }

        if NUMBERS.contains(character) {
            count_entropy_numbers_possitive = count_entropy_numbers_possitive + 1.0;
        }

        if SYMBOLS.contains(character) {
            count_entropy_symbols_possitive = count_entropy_symbols_possitive + 1.0;
        }
    }

    let entropy_general_possitive: f64 = entropy_calculos(
        count_entropy_general_possitive,
        chars_lenth - count_entropy_general_possitive,
    );
    let entropy_letters_possitive: f64 = entropy_calculos(
        count_entropy_letters_possitive,
        chars_lenth - count_entropy_letters_possitive,
    );
    let entropy_numbers_possitive: f64 = entropy_calculos(
        count_entropy_numbers_possitive,
        chars_lenth - count_entropy_numbers_possitive,
    );
    let entropy_symbols_possitive: f64 = entropy_calculos(
        count_entropy_symbols_possitive,
        chars_lenth - count_entropy_symbols_possitive,
    );

    let entropy_general_negative: f64 = entropy_calculos(
        chars_lenth - count_entropy_general_possitive,
        count_entropy_general_possitive,
    );
    let entropy_letters_negative: f64 = entropy_calculos(
        chars_lenth - count_entropy_letters_possitive,
        count_entropy_letters_possitive,
    );
    let entropy_numbers_negative: f64 = entropy_calculos(
        chars_lenth - count_entropy_numbers_possitive,
        count_entropy_numbers_possitive,
    );
    let entropy_symbols_negative: f64 = entropy_calculos(
        chars_lenth - count_entropy_symbols_possitive,
        count_entropy_symbols_possitive,
    );

    let entropy_general_percent_of_bin: f64 = (chars_lenth / entropy_general_possitive) * 100.0;
    let entropy_letters_percent_of_bin: f64 = (chars_lenth / entropy_letters_possitive) * 100.0;
    let entropy_numbers_percent_of_bin: f64 = (chars_lenth / entropy_numbers_possitive) * 100.0;
    let entropy_symbols_percent_of_bin: f64 = (chars_lenth / entropy_symbols_possitive) * 100.0;

    entropy_result.insert("entropy_general_possitive", entropy_general_possitive);
    entropy_result.insert("entropy_letters_possitive", entropy_letters_possitive);
    entropy_result.insert("entropy_numbers_possitive", entropy_numbers_possitive);
    entropy_result.insert("entropy_symbols_possitive", entropy_symbols_possitive);

    entropy_result.insert("entropy_general_negative", entropy_general_negative);
    entropy_result.insert("entropy_letters_negative", entropy_letters_negative);
    entropy_result.insert("entropy_numbers_negative", entropy_numbers_negative);
    entropy_result.insert("entropy_symbols_negative", entropy_symbols_negative);

    entropy_result.insert(
        "entropy_general_percent_of_bin",
        entropy_general_percent_of_bin,
    );
    entropy_result.insert(
        "entropy_letters_percent_of_bin",
        entropy_letters_percent_of_bin,
    );
    entropy_result.insert(
        "entropy_numbers_percent_of_bin",
        entropy_numbers_percent_of_bin,
    );
    entropy_result.insert(
        "entropy_symbols_percent_of_bin",
        entropy_symbols_percent_of_bin,
    );

    println!("🚧 {} | {:?}", chars_lenth, entropy_result);

    return entropy_result;
}

pub fn entropy_analysis(content: String, _debug: bool) -> bool {
    let mut final_entropy_general_possitive: f64 = 0.0;
    let mut final_entropy_letters_possitive: f64 = 0.0;
    let mut final_entropy_numbers_possitive: f64 = 0.0;
    let mut final_entropy_symbols_possitive: f64 = 0.0;
    let mut final_entropy_general_negative: f64 = 0.0;
    let mut final_entropy_letters_negative: f64 = 0.0;
    let mut final_entropy_numbers_negative: f64 = 0.0;
    let mut final_entropy_symbols_negative: f64 = 0.0;
    let mut final_entropy_general_percent_of_bin: f64 = 0.0;
    let mut final_entropy_letters_percent_of_bin: f64 = 0.0;
    let mut final_entropy_numbers_percent_of_bin: f64 = 0.0;
    let mut final_entropy_symbols_percent_of_bin: f64 = 0.0;

    let mut entropy_analysis: Vec<HashMap<&'static str, f64>> = Vec::new();
    let mut lines = content.lines();
    let mut lines_count: f64 = 0.0;

    // if lines_count == 0 {
    //     println!("🔴 [WARNING] :: Empty file");
    // }

    loop {
        match lines.next() {
            Some(line) => {
                let filtered_line = entropy_filter(line.to_string());
                entropy_analysis.push(filtered_line.clone());
                final_entropy_general_possitive =
                    final_entropy_general_possitive + filtered_line["entropy_general_possitive"];
                final_entropy_letters_possitive =
                    final_entropy_letters_possitive + filtered_line["entropy_letters_possitive"];
                final_entropy_numbers_possitive =
                    final_entropy_numbers_possitive + filtered_line["entropy_numbers_possitive"];
                final_entropy_symbols_possitive =
                    final_entropy_symbols_possitive + filtered_line["entropy_symbols_possitive"];
                final_entropy_general_negative =
                    final_entropy_general_negative + filtered_line["entropy_general_negative"];
                final_entropy_letters_negative =
                    final_entropy_letters_negative + filtered_line["entropy_letters_negative"];
                final_entropy_numbers_negative =
                    final_entropy_numbers_negative + filtered_line["entropy_numbers_negative"];
                final_entropy_symbols_negative =
                    final_entropy_symbols_negative + filtered_line["entropy_symbols_negative"];
                final_entropy_general_percent_of_bin = final_entropy_general_percent_of_bin
                    + filtered_line["entropy_general_percent_of_bin"];
                final_entropy_letters_percent_of_bin = final_entropy_letters_percent_of_bin
                    + filtered_line["entropy_letters_percent_of_bin"];
                final_entropy_numbers_percent_of_bin = final_entropy_numbers_percent_of_bin
                    + filtered_line["entropy_numbers_percent_of_bin"];
                final_entropy_symbols_percent_of_bin = final_entropy_symbols_percent_of_bin
                    + filtered_line["entropy_symbols_percent_of_bin"];
                lines_count = lines_count + 1.0;
                // println!("{:?} \n", filtered_line);
            }
            None => break,
        }
    }
    // Analysis by the lines

    let output = format!(
        r###"
        Entropy ascii general :: {}
        Entropy ascii numbers :: {}
        Entropy ascii symbols :: {}
        Entropy ascii letters :: {}
        File lines            :: {}

        ┌──────────────────────┬───────────────────────┐
        |       ascii          |        binary         |
        ├──────────────────────┼───────────────────────┤
        |          {}          |          {}           |
        ├──────────────────────┼───────────────────────┤
        |          {}          |          {}           |
        ├──────────────────────┼───────────────────────┤
        |          {}          |          {}           |
        ├──────────────────────┼───────────────────────┤
        |          {}          |          {}           |
        └──────────────────────┴───────────────────────┘
                
        "###,
        final_entropy_general_percent_of_bin / lines_count,
        final_entropy_numbers_percent_of_bin / lines_count,
        final_entropy_symbols_percent_of_bin / lines_count,
        final_entropy_letters_percent_of_bin / lines_count,
        lines_count,
        final_entropy_general_possitive / lines_count,
        final_entropy_general_negative / lines_count,
        final_entropy_letters_possitive / lines_count,
        final_entropy_letters_negative / lines_count,
        final_entropy_numbers_possitive / lines_count,
        final_entropy_numbers_negative / lines_count,
        final_entropy_symbols_possitive / lines_count,
        final_entropy_symbols_negative / lines_count,
    );

    println!("{}", output);

    return true;
}

pub fn advanced_entropy_scanner(file_path: &str, debug: bool) -> bool {
    read_file_to_hex(file_path, debug);
    true
}
