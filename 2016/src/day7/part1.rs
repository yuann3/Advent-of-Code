use anyhow::Result;
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
            todo!()
        }
    }
}

pub fn solve() -> Result<String> {
    let lines = read_lines("input/day7.in")?;
    if lines.is_empty() {
        return Ok(String::new());
    }

    let col_count = lines[0].len();
    let mut message = String::with_capacity(col_count);

    for pos in 0..col_count {
        let mut freq = [0u32; 26];

        for line in &lines {
            let ch = line.as_bytes()[pos] as char;
            if ch > 'a' && ch <= 'z' {
                let idx = (ch as u8 - b'a') as usize;
                freq[idx] += 1;
            }
        }

        let max_idx = (0..26).max_by_key(|&i| freq[i]).unwrap_or(0);
        let max_char = (b'a' + max_idx as u8) as char;
        message.push(max_char);
    }

    Ok(message)
}
