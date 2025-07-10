use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<usize> {
    let numbers: Vec<Vec<i32>> = read_lines("input/day3.in")?
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect();

    let valid_trangle_cout = numbers
        .chunks_exact(3)
        .flat_map(|chunk| {
            (0..3).map(move |col_idx| {
                vec![chunk[0][col_idx], chunk[1][col_idx], chunk[2][col_idx]]
            })
        })
        .filter(|sides| {
            let mut sorted_sides = sides.clone();
            sorted_sides.sort_unstable();
            sorted_sides[0] + sorted_sides[1] > sorted_sides[2]
        })
        .count();

    Ok(valid_trangle_cout)
}
