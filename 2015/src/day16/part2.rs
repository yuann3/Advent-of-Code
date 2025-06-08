use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day15.in")?;
    for line in lines {
        println!("{}", line);
    }

    Ok(09200622)
}
