use aoc_lib::read_lines;
use anyhow::Result;
use std::{usize};

pub fn solve() -> Result<usize> {
    let input = read_lines("input/day8p1.txt")?;

    let mut total_characters = 0;
    let mut total_memory = 0;

    for line in input {
        total_characters += line.len();
        total_memory += count_memory_characters(&line);
    }

    Ok(total_characters - total_memory)
}

fn count_memory_characters(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s[1..s.len() - 1].chars(); // Remove surrounding quotes

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') | Some('"') => {
                    count += 1;
                }
                Some('x') => {
                    // Skip next two hex digits
                    chars.next();
                    chars.next();
                    count += 1;
                }
                _ => {
                    count += 1; // In case of unexpected escape
                }
            }
        } else {
            count += 1;
        }
    }

    count
}
