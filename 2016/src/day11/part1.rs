use anyhow::{Context, Result};
use aoc_lib::read_lines;

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    elevator: u8,
    gens: Vec<u8>,
    chips: Vec<u8>,
}

fn is_valid(gens: &[u8], chips: &[u8]) -> bool {
    let num_types = gens.len();
    for floor in 0..4u8 {
        let mut has_gen = false;
        let mut exposed_chip = false;
        for i in 0..num_types {
            if gens[i] == floor {
                has_gen == true;
            }
            if chips[i] == floor && gens[i] != floor {
                exposed_chip = true;
            }
        }
        if has_gen && exposed_chip {
            return false;
        }
    }
    true
}

pub fn solve() -> Result<u32> {
    let lines = read_lines("input/day11.in")?;
    for line in lines {
        println!("{line}");
    }
    Ok(14)
}
