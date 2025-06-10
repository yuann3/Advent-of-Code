use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day20.in")?;
    let target: i32 = lines[0].trim().parse()?;

    let max_house = target / 10;

    let mut presents = vec![0; (max_house + 1) as usize];

    for elf in 1..=max_house {
        let mut house = elf;
        while house <= max_house {
            presents[house as usize] += 10 * elf;
            house += elf;
        }
    }

    for house in 1..max_house {
        if presents[house as usize] >= target {
            return Ok(house);
        }
    }

    Ok(-1)
}
