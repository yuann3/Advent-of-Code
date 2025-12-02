use aoc_lib::read_single_line;
use anyhow::{Context, Result};
use std::str::FromStr;

struct SearchRange {
    start: u64,
    end: u64,
}

impl FromStr for SearchRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .split_once('-')
            .context("Invalid range format: missing '-'")?;
        Ok(SearchRange {
            start: start_str.parse()?,
            end: end_str.parse()?,
        })
    }
}

pub fn solve() -> Result<u64> {
    let ranges: Vec<SearchRange> = read_single_line("input/day2.in", ",")?;

    let total_sum = ranges
        .iter()
        .flat_map(|r| r.start..=r.end)
        .filter(|&id| is_invalid_id(id))
        .sum();

    Ok(total_sum)
}

fn is_invalid_id(n: u64) -> bool {
    if n < 10 {
        return false;
    }

    // fast digit counting
    let mut digits = 0;
    let mut temp = n;
    while temp > 0 {
        temp /= 10;
        digits += 1;
    }

    if digits % 2 != 0 {
        return false;
    }

    // n = S * 10^k + S = S * (10^k + 1), k is half the number of digits
    let k = digits / 2;
    let divisor = 10u64.pow(k) + 1;

    n % divisor == 0
}
