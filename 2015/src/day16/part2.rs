use std::collections::HashMap;
use anyhow::{anyhow, Result};
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day16.in")?;
    let mut target: HashMap<String, i32> = HashMap::new();

    target.insert("children".to_string(), 3);
    target.insert("cats".to_string(), 7);
    target.insert("samoyeds".to_string(), 2);
    target.insert("pomeranians".to_string(), 3);
    target.insert("akitas".to_string(), 0);

    target.insert("vizslas".to_string(), 0);
    target.insert("goldfish".to_string(), 5);
    target.insert("trees".to_string(), 3);
    target.insert("cars".to_string(), 2);
    target.insert("perfumes".to_string(), 1);

    for line in lines {
        if let Some(sue_number) = parse_and_check_sue(&line, &target)? {
            return Ok(sue_number);
        }
    }

    anyhow::bail!("No match")
}

fn parse_and_check_sue(line: &str, target: &HashMap<String, i32>) -> Result<Option<i32>> {
    let parts: Vec<&str> = line.split(":").collect();
    let sue_number: i32 = parts[0]
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow!("error number"))?
        .parse()?;

    let attribute_str = parts[1..].join(": ");
    let mut sue_attributes: HashMap<String, i32> = HashMap::new();

    for attri_pair in attribute_str.split(", ") {
        let attri_parts: Vec<&str> = attri_pair.split(": ").collect();
        if attri_parts.len() == 2 {
            let compound = attri_parts[0].trim().to_string();
            let count = attri_parts[1].trim().parse()?;
            sue_attributes.insert(compound, count);
        }
    }

    for (compound, count) in sue_attributes {
        if let Some(&target_count) = target.get(&compound) {
            if !match_target(&compound, count, target_count) {
                return Ok(None);
            }
        }
    }

    Ok(Some(sue_number))
}

fn match_target(compound: &str, sue_count: i32, target_count: i32) -> bool {
    match compound {
        "cats" | "trees" => sue_count > target_count,
        "pomeranians" | "goldfish" => sue_count < target_count,
        _ => sue_count == target_count,
    }
}
