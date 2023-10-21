// MIT License

// Copyright (c) 2023 TH3MAID

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::core::messages::standard_messages;
use std::collections::HashMap;
use std::fs;

pub fn filter(line: &str) -> String {
    let mut formated = String::new();

    for symbol in line.chars() {
        if symbol == '"' {
            formated = format!("{}{}", formated, "");
        } else if symbol == '\\' {
            formated = format!("{}{}", formated, "\\\\");
        } else if symbol == '#' {
            formated = format!("{}{}", formated, "");
        } else {
            formated = format!("{}{}", formated, symbol);
        }
    }

    return formated.trim_start().to_string();
    
}

pub fn read_meow(path: &str, debug: bool) -> HashMap<String, String> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let mut result = HashMap::new();

    loop {
        match lines.next() {
            Some(line) => {
                let sys_args: Vec<String> = line.split_whitespace().map(String::from).collect();

                if debug == true {
                    standard_messages("debug", "Read meow output", "read_meow", "cute");
                    println!("{:?}", sys_args);
                }

                if sys_args.len() >= 4 {
                    let key = filter(&sys_args[1]);
                    let mut value = String::new();
                    for item in &sys_args[3..] {
                        value = format!("{} {}", value, item);
                    }
                    let val = filter(&value);
                    result.insert(key, val);
                }
            }
            None => break,
        }
    }

    return result;
}
