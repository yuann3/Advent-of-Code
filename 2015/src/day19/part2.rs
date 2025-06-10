use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day19.in")?;
    let (_replacements, molecule) = parse_input(&lines)?;
    let e_count = molecule
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .count();
    let rn_count = molecule.matches("Rn").count();
    let ar_count = molecule.matches("Ar").count();
    let y_count = molecule.matches('Y').count();

    // steps = E - Rn - Ar - 2*Y - 1
    let steps = e_count - rn_count - ar_count - 2 * y_count - 1;

    Ok(steps as i32)
}

fn parse_input(lines: &[String]) -> Result<(Vec<(String, String)>, String)> {
    let mut replacements = Vec::new();
    let mut molecule = String::new();
    for line in lines {
        if let Some(idx) = line.find(" => ") {
            let from = &line[..idx];
            let to = &line[idx + 4..];
            replacements.push((from.to_string(), to.to_string()));
        } else if !line.trim().is_empty() {
            molecule = line.clone();
        }
    }
    Ok((replacements, molecule))
}
