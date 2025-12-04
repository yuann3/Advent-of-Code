use anyhow::{Context, Result};
use aoc_lib::read_single_line;
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
        .into_iter()
        .flat_map(|r| r.start..=r.end)
        .filter(|&id| is_invalid_id(id))
        .sum();

    Ok(total_sum)
}

fn is_invalid_id(n: u64) -> bool {
    if n < 10 {
        return false;
    }

    let mut temp = n;
    let mut len = 0;
    while temp > 0 {
        temp /= 10;
        len += 1;
    }

    for k in 1..=(len / 2) {
        if len % k == 0 {
            let base_pow = 10u64.pow(k);
            let repetitions = len / k;

            let mut r = 0u64;
            for _ in 0..repetitions {
                if let Some(next_r) = r.checked_mul(base_pow) {
                    r = next_r + 1;
                } else {
                    r = 0;
                    break;
                }
            }

            if r > 0 && n % r == 0 {
                return true;
            }
        }
    }

    false
}
