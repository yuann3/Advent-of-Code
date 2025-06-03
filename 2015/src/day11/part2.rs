use aoc_lib::read_to_char;
use anyhow::Result;

pub fn solve() -> Result<usize> {
    let input = read_to_char("input/day11.in");
    for ichar in input {
        let output: String = ichar.iter().collect();
        println!("{}", output);
    }

    Ok(0)
}
