use anyhow::{Context, Result};
use aoc_lib::read_lines;
use regex::Regex;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day25.in")?;
    let input_line = lines.first().context("Input file is empty")?;

    let re = Regex::new(r"row (\d+), column (\d+)")?;
    let caps = re
        .captures(input_line)
        .context("Failed to parse row/column from input")?;
    let row: u64 = caps[1].parse()?;
    let col: u64 = caps[2].parse()?;

    let target_index = ((row + col - 2) * (row + col - 1) / 2) + col;

    let iterations = target_index - 1;

    let mut code = 20151125_u64;
    const MULTIPLIER: u64 = 252533;
    const DIVISOR: u64 = 33554393;

    for _ in 0..iterations {
        code = (code * MULTIPLIER) % DIVISOR;
    }

    Ok(code)
}
