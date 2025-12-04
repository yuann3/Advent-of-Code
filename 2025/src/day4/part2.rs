use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u32> {
    let grid: Vec<Vec<char>> = read_lines("input/day4.in")?
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let neighbors = (-1..=1)
                .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
                .filter(|&(dr, dc)| (dr, dc) != (0, 0))
                .filter(|(dr, dc)| {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    nr >= 0 && nc >= 0 
                        && (nr as usize) < rows 
                        && (nc as usize) < cols
                        && grid[nr as usize][nc as usize] == '@'
                })
                .count();

            if neighbors < 4 {
                count += 1;
            }
        }
    }

    Ok(count)
}
