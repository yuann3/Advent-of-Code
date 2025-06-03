use aoc_lib::read_lines;
use anyhow::Result;

pub fn solve() -> Result<i32> {
    let input = read_lines("input/day1.txt")?;
    let mut result: i32 = 0;
    for line in input {
        if let Ok(mass) = line.parse::<i32>() {
            let fuel = mass / 3 - 2;
            result += fuel;
        }
    }
    Ok(result)
}
