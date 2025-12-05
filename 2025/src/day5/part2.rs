use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day5.in")?;

    let mut ranges: Vec<(u64, u64)> = lines
        .iter()
        .take_while(|line| line.contains('-'))
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    ranges.sort_by_key(|&(lo, _)| lo);

    let mut merged: Vec<(u64, u64)> = vec![];
    for (lo, hi) in ranges {
        if let Some(last) = merged.last_mut() {
            if lo <= last.1 + 1 {
                last.1 = last.1.max(hi);
                continue;
            }
        }
        merged.push((lo, hi));
    }

    let count: u64 = merged.iter().map(|(lo, hi)| hi - lo + 1).sum();

    Ok(count)
}
