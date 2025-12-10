use anyhow::Result;
use aoc_lib::read_lines;

fn min_presses(line: &str) -> u32 {
    let target: u64 = line
        .split('[').nth(1).unwrap()
        .split(']').next().unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(0, |acc, (i, _)| acc | (1 << i));

    let buttons: Vec<u64> = line
        .split('(')
        .skip(1)
        .map(|s| {
            s.split(')').next().unwrap()
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .fold(0, |acc, bit| acc | (1 << bit))
        })
        .collect();

    let n = buttons.len();
    (0u64..(1 << n))
        .filter(|&mask| {
            (0..n)
                .filter(|&i| mask & (1 << i) != 0)
                .fold(0, |acc, i| acc ^ buttons[i]) == target
        })
        .map(|mask| mask.count_ones())
        .min()
        .unwrap()
}

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day10.in")?;
    let total: u32 = lines.iter().map(|l| min_presses(l)).sum();
    Ok(total as u64)
}
