use aoc_lib::read_lines;
use anyhow::{Context, Result};

pub fn solve() -> Result<i32> {
    let input = read_lines("input/day2.in")?;
    let mut safe_count = 0;

    for (line_num, line) in input.iter().enumerate() {
        let nums: Result<Vec<i32>, _> = line
            .split_whitespace()
            .enumerate()
            .map(|(num_index, num_str)| {
                num_str.parse::<i32>()
                    .with_context(|| format!("Failed to parse number '{}' (position {}) on line {}", num_str, num_index + 1, line_num + 1))
            })
            .collect();
        
        let nums = nums?;

        if is_safe(&nums) {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

fn is_safe(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let increasing = nums[0] < nums[1];

    for pair in nums.windows(2) {
        let current = pair[0];
        let next = pair[1];

        if !is_valid_difference(current, next) {
            return false;
        }

        if increasing && current >= next {
            return false;
        }

        if !increasing && current <= next {
            return false;
        }
    }
    true
}

fn is_valid_difference(a: i32, b: i32) -> bool {
    let diff = (b - a).abs();
    diff >= 1 && diff <= 3
}
