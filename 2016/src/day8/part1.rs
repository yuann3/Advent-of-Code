use anyhow::{Context, Result};
use aoc_lib::read_lines;

fn has_abba(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len().saturating_sub(3) {
        if chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1] {
            return true;
        }
    }
    false
}

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

pub fn solve() -> Result<usize> {
    let ips = read_lines("input/day7.in").context("Failed to read input file")?;

    let count = ips
        .iter()
        .filter(|ip| {
            let (supernets, hypernets) = parse_ip(ip);
            supernets.iter().any(|s| has_abba(s)) && hypernets.iter().all(|h| !has_abba(h))
        })
        .count();

    Ok(count)
}
