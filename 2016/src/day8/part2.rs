use anyhow::{Context, Result};
use aoc_lib::read_lines;

fn parse_ip(ip: &str) -> (Vec<String>, Vec<String>) {
    let mut supernets = Vec::new();
    let mut hypernets = Vec::new();
    let mut current = String::new();
    let mut inside = false;

    for c in ip.chars() {
        match c {
            '[' => {
                if !inside {
                    supernets.push(current);
                    current = String::new();
                    inside = true;
                }
            }
            ']' => {
                if inside {
                    hypernets.push(current);
                    current = String::new();
                    inside = false;
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        if inside {
            hypernets.push(current);
        } else {
            supernets.push(current);
        }
    }
    (supernets, hypernets)
}

fn supports_ssl(supernets: &[String], hypernets: &[String]) -> bool {
    for sup in supernets {
        let chars: Vec<char> = sup.chars().collect();
        for i in 0..chars.len().saturating_sub(2) {
            if chars[i] == chars[i + 2] && chars[i] != chars[i + 1] {
                let a = chars[i];
                let b = chars[i + 1];
                let bab: String = [b, a, b].iter().collect();
                if hypernets.iter().any(|h| h.contains(&bab)) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn solve() -> Result<usize> {
    let ips = read_lines("input/day7.in").context("Failed to read input file")?;

    let count = ips
        .iter()
        .filter(|ip| {
            let (supernets, hypernets) = parse_ip(ip);
            supports_ssl(&supernets, &hypernets)
        })
        .count();

    Ok(count)
}
