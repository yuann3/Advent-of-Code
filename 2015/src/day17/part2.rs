
use anyhow::Result;
use aoc_lib::read_one_per_line;

pub fn solve() -> Result<i32> {
    let containers: Vec<i32> = read_one_per_line("input/day17.in")?;
    let target = 150;

    let mut count = 0;
    let n = containers.len();

    for mask in 1..(1 << n) {
        let mut sum = 0;

        for i in 0..n {
            if mask & (1 << i) != 0 {
                sum += containers[i];
                if sum > target {
                    break;
                }
            }
        }

        if sum == target {
            count += 1;
        }
    }

    Ok(count)
}
