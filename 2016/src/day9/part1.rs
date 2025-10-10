use anyhow::{Context, Result};
use aoc_lib::read_lines;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day9.in")?;
    let input = lines.into_iter().next().context("Empty input")?;

    let mut length: u64 = 0;
    let mut i: usize = 0;

    while i < input.len() {
        let ch = input.as_bytes()[i] as char;
        if ch == '(' {
            let mut j = i + 1;
            while j < input.len() && input.as_bytes()[j] as char != ')' {
                j += 1;
            }
            if j == input.len() {
                return Err(anyhow::anyhow!("Unclosed marker at {}", i));
            }
            let marker_str = &input[(i + 1)..j];
            let parts: Vec<&str> = marker_str.split('x').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid marker: {}", marker_str));
            }
            let data_len: usize = parts[0]
                .parse()
                .context(format!("Parse len: {}", parts[0]))?;
            let repeat: u64 = parts[1]
                .parse()
                .context(format!("Parse rep: {}", parts[1]))?;
            let data_start = j + 1;
            let data_end = data_start + data_len;
            if data_end > input.len() {
                return Err(anyhow::anyhow!(
                    "Data overrun: need {} from {}, total {}",
                    data_len,
                    data_start,
                    input.len()
                ));
            }
            length += data_len as u64 * repeat;
            i = data_end;
        } else {
            length += 1;
            i += 1;
        }
    }

    Ok(length)
}
