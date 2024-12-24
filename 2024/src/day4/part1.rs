use aoc_lib::read_lines;
use std::io;

struct Grid {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: Vec<String>) -> Self {
        let data: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        let rows = data.len();
        let cols = data[0].len();
        Grid { data, rows, cols }
    }

    fn check_pattern(&self, row: usize, col: usize, dx: i32, dy: i32) -> bool {
        // dx and dy represent direction:
        // (1,0) = right, (-1,0) = left
        // (0,1) = down, (0,-1) = up
        // (1,1) = diagonal down-right, etc.
        let pattern = "XMAS";

        // check if pattern would go out of bounds
        for i in 0..4 {
            // XMAS is 4 chars long
            let new_row = row as i32 + dy * i;
            let new_col = col as i32 + dx * i;

            if new_row < 0
                || new_row >= self.rows as i32
                || new_col < 0
                || new_col >= self.cols as i32
            {
                return false;
            }

            if self.data[new_row as usize][new_col as usize]
                != pattern.chars().nth(i.try_into().unwrap()).unwrap()
            {
                return false;
            }
        }
        true
    }

    fn count_xmas(&self) -> i32 {
        let mut count = 0;
        let directions = [
            (1, 0),
            (-1, 0), // horizontal
            (0, 1),
            (0, -1), // vertical
            (1, 1),
            (-1, -1), // diagonal
            (1, -1),
            (-1, 1), // other diagonal
        ];

        for row in 0..self.rows {
            for col in 0..self.cols {
                for (dx, dy) in directions.iter() {
                    if self.check_pattern(row, col, *dx, *dy) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

pub fn solve() -> io::Result<i32> {
    let input = read_lines("input/day4.in")?;
    let grid = Grid::new(input);
    Ok(grid.count_xmas())
}
