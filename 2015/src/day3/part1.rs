use aoc_lib::read_to_char;
use std::collections::HashSet;
use std::io;

#[derive(Hash, Clone, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

pub fn solve() -> io::Result<usize> {
    let input = read_to_char("input/day3p1.txt")?;

    let mut santa_coordinate = Coordinate { x: 0, y: 0 };
    let mut visited = HashSet::new();

    visited.insert(santa_coordinate.clone());

    for c in input {
        match c {
            '^' => santa_coordinate.y += 1,
            'v' => santa_coordinate.y -= 1,
            '>' => santa_coordinate.x += 1,
            '<' => santa_coordinate.x -= 1,
            _ => continue,
        }
        visited.insert(santa_coordinate.clone());
    }

    Ok(visited.len())
}
