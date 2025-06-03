use aoc_lib::read_to_char;
use anyhow::Result;
use std::{usize};

pub fn solve() -> Result<usize> {
    let input = read_to_char("input/day1p1.txt")?;

    let mut floor: i32 = 0;

    for (i, c) in input.iter().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }

        if floor == -1 {
            return Ok(i + 1);
        }
    }
    Ok(0)
}
