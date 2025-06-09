
use anyhow::Result;
use aoc_lib::read_one_per_line;

pub fn solve() -> Result<i32> {
    let containers: Vec<i32> = read_one_per_line("input/day17.in")?;
    let target = 150;
    let n = containers.len();

    let mut min_containers = n + 1;
    let mut valid_combinations = Vec::new();

    for mask in 1..(1 << n) {
        let mut sum = 0;
        let mut container_count = 0;

        for i in 0..n {
            if mask & (1 << i) != 0 {
                sum += containers[i];
                container_count += 1;

                if sum > target {
                    break;
                }
                if container_count > min_containers {
                    break;
                }
            }
        }

        if sum == target {
            if container_count < min_containers {
                min_containers = container_count;
                valid_combinations.clear();
                valid_combinations.push(mask);
            } else if container_count ==  min_containers {
                valid_combinations.push(mask);
            }
        }
    }

    Ok(valid_combinations.len() as i32)
}
