use anyhow::{anyhow, Context, Result};
use aoc_lib::read_lines;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

pub fn solve() -> Result<usize> {
    let lines = read_lines("input/day8.in")?;

    let mut screen: Vec<Vec<u8>> = vec![vec![0; WIDTH]; HEIGHT];

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "rect" => {
                let ab: Vec<&str> = parts[1].split('x').collect();
                let a: usize = ab[0].parse().context("Failed to parse rect width")?;
                let b: usize = ab[1].parse().context("Failed to parse rect height")?;
                for r in 0..b {
                    for c in 0..a {
                        screen[r][c] = 1;
                    }
                }
            }
            "rotate" => {
                let kind = parts[1];
                let coord: Vec<&str> = parts[2].split('=').collect();
                let idx: usize = coord[1].parse().context("Failed to parse rotate index")?;
                let by: usize = parts[4].parse().context("Failed to parse rotate amount")?;
                if kind == "row" {
                    let by = by % WIDTH;
                    screen[idx].rotate_right(by);
                } else if kind == "column" {
                    let by = by % HEIGHT;
                    let mut col: Vec<u8> = (0..HEIGHT).map(|r| screen[r][idx]).collect();
                    col.rotate_right(by);
                    for r in 0..HEIGHT {
                        screen[r][idx] = col[r];
                    }
                }
            }
            _ => return Err(anyhow!("Invalid instruction: {}", line)),
        }
    }

    let lit_count: usize = screen.iter().flatten().map(|&p| p as usize).sum();
    Ok(lit_count)
}
