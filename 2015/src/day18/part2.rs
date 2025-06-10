use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day18.in")?;

    let mut grid: Vec<Vec<bool>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    force_corner_on(&mut grid, rows, cols);

    for _ in 0..100 {
        grid = simulate(&grid, rows, cols);
        force_corner_on(&mut grid, rows, cols);
    }

    let count = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&light| light)
        .count() as i32;

    Ok(count)
}

fn simulate(grid: &[Vec<bool>], rows: usize, cols: usize) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let neighbors = count_neighbors(grid, i, j, rows, cols);
            let current = grid[i][j];

            new_grid[i][j] = match (current, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_grid
}

fn count_neighbors(grid: &[Vec<bool>], row: usize, col: usize, rows: usize, cols: usize) -> usize {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    directions
        .iter()
        .filter_map(|&(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_row < rows as isize &&
               new_col >= 0 && new_col < cols as isize {
                Some(grid[new_row as usize][new_col as usize])
            } else {
                None
            }
        })
        .filter(|&light| light)
        .count()
}

fn force_corner_on(grid: &mut [Vec<bool>], rows: usize, cols: usize) {
    grid[0][0] = true;
    grid[0][cols - 1] = true;
    grid[rows - 1][0] = true;
    grid[rows - 1][cols - 1] = true;
}
