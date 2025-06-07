use anyhow::Result;
use aoc_lib::read_lines;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
}

impl Ingredient {
    fn parse_line(line: &str) -> Result<Self> {
        todo!()
    }
}

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day15.in")?;
    for line in lines {
        println!("{}", line);
    }

    Ok(09200622)
}
