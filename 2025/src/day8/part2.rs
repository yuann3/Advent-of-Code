use anyhow::Result;
use aoc_lib::read_lines;
use std::collections::HashSet;

pub fn solve() -> Result<i32> {
    let grid: Vec<Vec<char>> = read_lines("input/day7.in")?
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap();

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);
    let mut splits = 0;

    for row in 1..rows {
        let mut next_beams: HashSet<usize> = HashSet::new();

        for &col in &beams {
            match grid[row][col] {
                '^' => {
                    splits += 1;
                    if col > 0 {
                        next_beams.insert(col - 1);
                    }
                    if col + 1 < cols {
                        next_beams.insert(col + 1);
                    }
                }
                _ => {
                    next_beams.insert(col);
                }
            }
        }

        beams = next_beams;
    }

    Ok(splits)
}
