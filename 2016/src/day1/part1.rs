use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let line = &read_lines("input/day1.in")?[0];

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: i32 = 0; // 0: N, 1: E, 2: S, 3: W

    for instruction in line.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let blocks: i32 = instruction[1..].parse()?;

        match turn {
            'R' => dir = (dir + 1) % 4,
            'L' => dir = (dir + 3) % 4,
            _ => unreachable!(),
        }

        match dir {
            0 => y += blocks, // North
            1 => x += blocks, // East
            2 => y -= blocks, // South
            3 => x -= blocks, // West
            _ => unreachable!(),
        }
    }

    Ok(x.abs() + y.abs())
}
