use std::collections::HashSet;
use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day19.in")?;

    let (replacements, molecule) = parse_input(&lines)?;

    let distinct_molecules = generate_replacement(&molecule, &replacements);

    Ok(distinct_molecules.len() as i32)
}

fn parse_input(lines: &[String]) -> Result<(Vec<(String, String)>, String)> {
    let mut replacements = Vec::new();
    let mut molecule = String::new();

    for line in lines {
        if line.contains(" => ") {
            let parts: Vec<&str> = line.split(" => ").collect();
            if parts.len() == 2 {
                replacements.push((parts[0].to_string(), parts[1].to_string()));
            }
        } else if !line.trim().is_empty() {
            molecule = line.clone();
        }
    }

    Ok((replacements, molecule))
}

fn generate_replacement(molecule: &str, replacements: &[(String, String)]) -> HashSet<String> {
    let mut distinct_molecules = HashSet::new();

    for (from, to) in replacements {
        let mut start = 0;

        while let Some(pos) = molecule[start..].find(from) {
            let absolute_pos = start + pos;

            let before = &molecule[..absolute_pos];
            let after = &molecule[absolute_pos + from.len()..];
            let new_molecule = format!("{}{}{}", before, to, after);

            distinct_molecules.insert(new_molecule);
            start = absolute_pos + 1;
        }
    }

    distinct_molecules
}
