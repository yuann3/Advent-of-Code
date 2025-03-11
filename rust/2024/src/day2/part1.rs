use aoc_lib::read_lines;
use std::io;

pub fn solve() -> io::Result<i32> {
    let input = read_lines("input/day2.in")?;
    let mut safe_count = 0;

    for line in input {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

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
