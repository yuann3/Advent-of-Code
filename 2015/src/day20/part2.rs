use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day20.in")?;
    let target: i32 = lines[0].trim().parse()?;

    let max_house = target / 11;

    let mut presents = vec![0; (max_house + 1) as usize];

    for elf in 1..=max_house {
        let mut visits = 0;
        let mut house = elf;
        while visits <= 50 && house <= max_house {
            presents[house as usize] += 11 * elf;
            house += elf;
            visits += 1;
        }
    }

    for house in 1..max_house {
        if presents[house as usize] >= target {
            return Ok(house);
        }
    }

    Ok(-1)
}
