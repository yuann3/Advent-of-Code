use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<String> {
    let lines = read_lines("input/day2.in")?;
    let mut x: i32 = 1;
    let mut y: i32 = 1;
    let mut code = String::new();

    for line in lines {
        for instruction in line.chars() {
           match instruction {
               'U' => y = (y - 1).max(0),
               'D' => y = (y + 1).min(2),
               'L' => x = (x - 1).max(0),
               'R' => x = (x + 1).min(2),
               _ => (),
           }
        }

        let digit = 1 + x + y * 3;
        code.push_str(&digit.to_string());
    }

    Ok(code)
}
