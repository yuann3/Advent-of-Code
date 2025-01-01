use aoc_lib::read_lines;
use std::{io, usize};

pub fn solve() -> io::Result<usize> {
    let inputs = read_lines("input/day8.in")?;

    let mut sum: usize = 0;
    for line in inputs {
        let char_in_code = line.len();
        let char_in_mem = count_mem_chars(&line);

        sum += char_in_code - char_in_mem;
    }
    Ok(sum)
}

fn count_mem_chars(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars().peekable();

    chars.next();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(&next_char) = chars.peek() {
                    match next_char {
                        '\\' | '"' => {
                            chars.next();
                            count += 1;
                        }
                        'x' => {
                            chars.next();
                            if let (Some(hex1), Some(hex2)) = (chars.next(), chars.next()) {
                                count += 1;
                            }
                        }
                        _ => count += 1,
                    }
                }
            }
            '"' => {
                continue;
            }
            _ => count += 1,
        }
    }

    count
}
