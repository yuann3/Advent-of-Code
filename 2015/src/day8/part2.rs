use aoc_lib::read_lines;
use anyhow::Result;
use std::{usize};

pub fn solve() -> Result<usize> {
    let input = read_lines("input/day8p1.txt")?;

    let mut original_length = 0;
    let mut encoded_length = 0;

    for line in input {
        original_length += line.len();
        encoded_length += encode_string(&line).len();
    }

    Ok(encoded_length - original_length)
}

fn encode_string(s: &str) -> String {
    let mut encoded = String::from("\"");
    
    for c in s.chars() {
        match c {
            '"' => encoded.push_str("\\\""),
            '\\' => encoded.push_str("\\\\"),
            _ => encoded.push(c),
        }
    }
    
    encoded.push('"');
    encoded
}
