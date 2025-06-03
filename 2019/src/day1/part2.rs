use aoc_lib::read_lines;
use anyhow::Result;

fn calculate_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + calculate_fuel(fuel)
    }
}

/// part2
pub fn solve() -> Result<i32> {
    let input = read_lines("input/day1.txt")?;
    let mut result: i32 = 0;

    for line in input {
        if let Ok(mass) = line.parse::<i32>() {
            result += calculate_fuel(mass);
        }
    }

    Ok(result)
}
