use aoc_lib::read_lines;
use anyhow::{Context, Result};
use std::iter::zip;

pub fn solve() -> Result<i32> {
    let input: Vec<String> = read_lines("input/day1.in")?;

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for (line_num, line) in input.iter().enumerate() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if let (Some(left), Some(right)) = (numbers.get(0), numbers.get(1)) {
            let left_num = left.parse::<i32>()
                .with_context(|| format!("Failed to parse left number '{}' on line {}", left, line_num + 1))?;
            let right_num = right.parse::<i32>()
                .with_context(|| format!("Failed to parse right number '{}' on line {}", right, line_num + 1))?;
            
                left_numbers.push(left_num);
                right_numbers.push(right_num);
        } else {
            return Err(anyhow::anyhow!("Invalid format on line {}: expected two numbers", line_num + 1));
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let result: i32 = zip(left_numbers, right_numbers)
        .map(|(left, right)| left.abs_diff(right) as i32)
        .sum();

    Ok(result)
}
