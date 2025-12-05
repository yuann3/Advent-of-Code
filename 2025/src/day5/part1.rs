use anyhow::Result;
use aoc_lib::read_lines_keep_empty;

pub fn solve() -> Result<u32> {
    let lines = read_lines_keep_empty("input/day5.in")?;
    let mut sections = lines.split(|line| line.is_empty());

    let ranges: Vec<(u64, u64)> = sections
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let ingredients: Vec<u64> = sections
        .next()
        .unwrap()
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let count = ingredients
        .iter()
        .filter(|&&id| ranges.iter().any(|&(lo, hi)| id >= lo && id <= hi))
        .count();

    Ok(count as u32)
}
