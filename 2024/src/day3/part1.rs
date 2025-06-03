use aoc_lib::read_lines;
use anyhow::{Context, Result};
use regex::Regex;

pub fn solve() -> Result<i32> {
    let input = read_lines("input/day3.in")?;
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .context("Failed to create regex pattern")?;

    let mut total = 0;

    for (line_num, line) in input.iter().enumerate() {
        for capture in pattern.captures_iter(&line) {
            let x = capture[1].parse::<i32>()
                .with_context(|| format!("Failed to parse first number '{}' on line {}", &capture[1], line_num + 1))?;
            let y = capture[2].parse::<i32>()
                .with_context(|| format!("Failed to parse second number '{}' on line {}", &capture[2], line_num + 1))?;
            total += x * y;
        }
    }

    Ok(total)
}
