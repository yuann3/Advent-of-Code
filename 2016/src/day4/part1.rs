use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day4.in")?;

    let sum = lines
        .iter()
        .filter_map(|line| {
            let last_dash_pos = line.rfind('-')?;
            let name = &line[..last_dash_pos];
            let rest = &line[last_dash_pos + 1..];

            let bracket_pos = rest.find('[')?;
            let sector_id_str = &rest[..bracket_pos];
            let checksum = &rest[bracket_pos + 1..rest.len() - 1];

            let sector_id: i32 = sector_id_str.parse().ok()?;

            let mut freqs = [0; 26];
            for c in name.chars() {
                if c.is_ascii_lowercase() {
                    freqs[(c as u8 - b'a') as usize] += 1;
                }
            }

            let mut counts: Vec<(char, i32)> = freqs
                .iter()
                .enumerate()
                .filter(|&(_, &count)| count > 0)
                .map(|(i, &count)| ((b'a' + i as u8) as char, count))
                .collect();

            counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

            let generated_checksum: String = counts.iter().take(5).map(|(c, _)| *c).collect();

            if generated_checksum == checksum {
                Some(sector_id)
            } else {
                None
            }
        })
        .sum();

    Ok(sum)
}
