use anyhow::{Context, Result};
use aoc_lib::read_lines;
use std::collections::HashMap;

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

fn parse_input(lines: &[String]) -> Result<(usize, Vec<u8>, Vec<u8>)> {
    let mut element_map: HashMap<String, usize> = HashMap::new();
    let mut next_id = 0;
    let mut gens: Vec<u8> = Vec::new();
    let mut chips: Vec<u8> = Vec::new();
    for (floor, line) in (0..4).zip(lines.iter()) {
        let content = match line.split("contains ").nth(1) {
            Some(c) => c.trim_end_matches('.'),
            None => continue,
        };
        if content == "nothing relevant" {
            continue;
        }
        let content = content.replace("and " , ", ");
        let items: Vec<&str> = content.split(", ").collect();
        for item in items {
            let words: Vec<&str> = item.split_whitespace().collect();
            if words.len() < 2 {
                continue;
            }
            let name = words[1];
            let kind = words[2];
            if kind == "generator" {
                let elem = name.to_string();
                let if = *element_map.entry(elem).or_insert_with(|| {
                    let if = next_id;
                    next_id += 1;
                    id
                });
                if gens.len() <= id {
                    gens.resize(id + 1, 0);
                }
                gens[id] = floor as u8;
            } else if kind == "microchip" {

            }
        }
    }
}

pub fn solve() -> Result<u32> {
    let lines = read_lines("input/day11.in")?;
    for line in lines {
        println!("{line}");
    }
    Ok(14)
}
