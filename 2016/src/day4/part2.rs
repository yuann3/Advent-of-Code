use anyhow::{anyhow, Result};
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day4.in")?;

    let sector_id = lines
        .iter()
        .find_map(|line| {
            let last_dash_pos = line.rfind('-')?;
            let name = &line[..last_dash_pos];
            let rest = &line[last_dash_pos + 1..];

            let bracket_pos = rest.find('[')?;
            let sector_id: i32 = rest[..bracket_pos].parse().ok()?;
            let checksum = &rest[bracket_pos + 1..rest.len() - 1];

            let mut freqs = [0; 26];
            for c in name.chars().filter(|&c| c != '-') {
                freqs[(c as u8 - b'a') as usize] += 1;
            }

            let mut counts: Vec<(char, i32)> = freqs
                .iter()
                .enumerate()
                .map(|(i, &count)| ((b'a' + i as u8) as char, count))
                .collect();

            counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

            let generated_checksum: String = counts.iter().take(5).map(|(c, _)| *c).collect();

            if generated_checksum == checksum {
                let shift = (sector_id % 26) as u8;
                let decrypted_name: String = name
                    .chars()
                    .map(|c| {
                        if c == '-' {
                            ' '
                        } else {
                            let base = c as u8 - b'a';
                            let rotated = (base + shift) % 26;
                            (rotated + b'a') as char
                        }
                    })
                    .collect();

                if decrypted_name.contains("northpole object") {
                    return Some(sector_id);
                }
            }

            None
        })
        .ok_or_else(|| anyhow!("North Pole object storage room not found"))?;

    Ok(sector_id)
}
