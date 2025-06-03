use aoc_lib::read_to_char;
use std::collections::HashSet;
use anyhow::Result;

#[derive(Hash, Clone, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

pub fn solve() -> Result<usize> {
    let input = read_to_char("input/day3p1.txt")?;

    let mut santa_coordinate = Coordinate { x: 0, y: 0 };
    let mut robot_coordinate = Coordinate { x: 0, y: 0 };
    let mut visited = HashSet::new();

    visited.insert(santa_coordinate.clone());

    for (i, c) in input.into_iter().enumerate() {
        let coord = if i % 2 == 0 {
            &mut santa_coordinate
        } else {
            &mut robot_coordinate
        };

        match c {
            '^' => coord.y += 1,
            'v' => coord.y -= 1,
            '>' => coord.x += 1,
            '<' => coord.x -= 1,
            _ => continue,
        }
        visited.insert(coord.clone());
    }

    Ok(visited.len())
}
