use aoc_lib::read_lines;
use std::{io, usize};

pub fn solve() -> io::Result<usize> {
    let inputs = read_lines("input/day8.in")?;

    let mut sum: usize = 0;
    for line in inputs {
        let char_in_code = line.len();
        let char_encoded = count_encoded_chars(&line);

        sum += char_encoded - char_in_code;
    }
    Ok(sum)
}

fn count_encoded_chars(s: &str) -> usize {
    let mut count = 2;
    let chars = s.chars();

    for c in chars {
        match c {
            '"' | '\\' => count += 2,
            _ => count += 1,
        }
    }

    count
}
