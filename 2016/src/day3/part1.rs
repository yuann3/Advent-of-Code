use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<usize> {
    let lines = read_lines("input/day3.in")?;

    let valid_trangle_cout = lines
        .iter()
        .filter_map(|line| {
            let mut sides: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if sides.len() == 3 {
                sides.sort_unstable();
                if sides[0] + sides[1] > sides[2] {
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .count();

    Ok(valid_trangle_cout)
}
