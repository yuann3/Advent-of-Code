use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i64> {
    let lines = read_lines("input/day1.in")?;

    let (_, clicks) = lines.iter().fold((50i64, 0i64), |(pos, clicks), line| {
        let (dir, amt_str) = line.split_at(1);
        let amt = amt_str.parse::<i64>().unwrap_or(0);

        match dir {
            "R" => {
                let next = pos + amt;
                let new_clicks = next.div_euclid(100) - pos.div_euclid(100);
                (next, clicks + new_clicks)
            }
            "L" => {
                let next = pos - amt;
                let new_clicks = (pos - 1).div_euclid(100) - (next - 1).div_euclid(100);
                (next, clicks + new_clicks)
            }
            _ => (pos, clicks),
        }
    });

    Ok(clicks)
}
