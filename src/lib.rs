pub mod day4;
pub mod day6;

use std::fs::read_to_string;

pub fn read_input(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
