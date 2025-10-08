use anyhow::Result;
use aoc_lib::read_lines;
use std::collections::HashSet;

pub fn solve() -> Result<i32> {
    let line = &read_lines("input/day1.in")?[0];

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: i32 = 0; // 0: N, 1: E, 2: S, 3: W

    let mut visited = HashSet::new();
    visited.insert((x, y));

    for instruction in line.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let blocks: i32 = instruction[1..].parse()?;

        match turn {
            'R' => dir = (dir + 1) % 4,
            'L' => dir = (dir + 3) % 4,
            _ => unreachable!(),
        }

        for _ in 0..blocks {
            match dir {
                0 => y += 1, // North
                1 => x += 1, // East
                2 => y -= 1, // South
                3 => x -= 1, // West
                _ => unreachable!(),
            }

            if !visited.insert((x, y)) {
                return Ok(x.abs() + y.abs());
            }
        }
    }
    unreachable!()
}
