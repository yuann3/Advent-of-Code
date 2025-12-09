use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day9.in")?;

    let points: Vec<(i64, i64)> = lines
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            max_area = max_area.max(width * height);
        }
    }

    Ok(max_area as u64)
}
