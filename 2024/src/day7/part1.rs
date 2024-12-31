use aoc_lib::read_lines;
use std::io;

pub fn solve() -> io::Result<i32> {
    let input = read_lines("input/day7.in")?;
    for (i, line) in input.iter().take(10).enu {
        println!("line: {}, {:?}", i, line);
    }
    Ok(0)
}
