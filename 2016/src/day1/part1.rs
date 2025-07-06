use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let line = &read_lines("input/day1.in")?[0];

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: i32 = 0;

    for instruction in line.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let blocks: i32 = instruction[1..].parse()?;

        match turn {
            'R' => dir = (dir + 1) % 4,
            'L' => dir = (dir + 3) % 4,
            _ => unreachable!(),
        }

        match dir {
            1 => y += blocks,
            2 => x += blocks,
            3 => y -= blocks,
            4 => x -= blocks,
            _ => unreachable!(),
        }

    }

    Ok(x.abs() + y.abs())
}
