use aoc_lib::read_lines;
use core::panic;
use std::{collections::HashSet, io};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn get_position_ahead(&self) -> Option<(usize, usize)> {
        let (row, col) = self.position;
        match self.direction {
            Direction::North => row.checked_sub(1).map(|r| (r, col)),
            Direction::South => Some((row + 1, col)),
            Direction::East => Some((row, col + 1)),
            Direction::West => col.checked_sub(1).map(|c| (row, c)),
        }
    }
}

pub fn solve() -> io::Result<i32> {
    let input = read_lines("input/day6.in")?;
    let (grid, guard) = parse_input(input);
    Ok(results(guard, &grid))
}

fn results(mut guard: Guard, grid: &Vec<Vec<char>>) -> i32 {
    let mut visited_positions = HashSet::new();
    let mut visited_states = HashSet::new();

    visited_positions.insert(guard.position);

    loop {
        let state = (guard.position, guard.direction);
        if visited_states.contains(&state) {
            break;
        }
        visited_states.insert(state);

        let next_pos = guard.get_position_ahead();

        match next_pos {
            Some(pos) => {
                if is_in_bounds(pos, grid) {
                    if grid[pos.0][pos.1] == '#' {
                        guard.turn_right();
                    } else {
                        guard.position = pos;
                        visited_positions.insert(guard.position);
                    }
                } else {
                    break;
                }
            }
            None => {
                break;
            }
        }
    }

    visited_positions.len() as i32
}

fn is_in_bounds(position: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    let (r, c) = position;
    r < grid.len() && c < grid[0].len()
}

fn parse_input(lines: Vec<String>) -> (Vec<Vec<char>>, Guard) {
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '^' {
                return (
                    grid,
                    Guard {
                        position: (row, col),
                        direction: Direction::North,
                    },
                );
            }
        }
    }
    panic!("Could not find guard starting position ('^') in the input.");
}

#[test]
fn test_guard_movement() {
    let input = vec![
        "....#.....".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        "..#.......".to_string(),
        ".......#..".to_string(),
        "..........".to_string(),
        ".#..^.....".to_string(),
        "........#.".to_string(),
        "#.........".to_string(),
        "......#...".to_string(),
    ];
    let (grid, guard) = parse_input(input);
    // Should match puzzle example => 41
    assert_eq!(results(guard, &grid), 41);
}
