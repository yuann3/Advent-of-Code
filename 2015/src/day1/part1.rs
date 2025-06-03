use aoc_lib::read_to_char;
use anyhow::Result;

pub fn solve() -> Result<i32> {
    let input = read_to_char("input/day1p1.txt")?;

    let mut floor: i32 = 0;

    for (_i, c) in input.iter().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
    }
    Ok(floor)
}
