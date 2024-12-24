use aoc_lib::read_lines;
use regex::Regex;
use std::io;

pub fn solve() -> io::Result<i32> {
    let input = read_lines("input/day3.in")?;
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total = 0;

    for line in input {
        for capture in pattern.captures_iter(&line) {
            if let (Ok(x), Ok(y)) = (capture[1].parse::<i32>(), capture[2].parse::<i32>()) {
                total += x * y;
            }
        }
    }

    Ok(total)
}
