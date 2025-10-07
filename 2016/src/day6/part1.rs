use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<String> {
    let lines = read_lines("input/day6.in")?;
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
