use aoc_lib::read_lines;
use std::{io, iter::zip};

pub fn solve() -> io::Result<i32> {
    let input: Vec<String> = read_lines("input/day1.in")?;

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for line in input {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if let (Some(left), Some(right)) = (numbers.get(0), numbers.get(1)) {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<i32>(), right.parse::<i32>()) {
                left_numbers.push(left_num);
                right_numbers.push(right_num);
            }
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let result: i32 = zip(left_numbers, right_numbers)
        .map(|(left, right)| left.abs_diff(right) as i32)
        .sum();

    Ok(result)
}
