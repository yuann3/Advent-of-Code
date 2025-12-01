use aoc_lib::read_lines;
use anyhow::Result;

pub fn solve() -> Result<usize> {
    let lines = read_lines("input/day1.in")?;

    // start at 50, apply rotation, check if resulting position is 0
    let count = lines
        .iter()
        .scan(50i32, |pos, line| {
            let (dir, amount_str) = line.split_at(1);
            let amount = amount_str.parse::<i32>().ok()?;

            *pos = match dir {
                "L" => (*pos - amount).rem_euclid(100),
                "R" => (*pos + amount) % 100,
                _ => *pos,
            };

            Some(*pos)
        })
        .filter(|&pos| pos == 0)
        .count();

    Ok(count)
}
