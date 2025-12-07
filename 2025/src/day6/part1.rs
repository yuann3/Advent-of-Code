use anyhow::Result;
use aoc_lib::read_lines_keep_empty;

pub fn solve() -> Result<u64> {
    let lines = read_lines_keep_empty("input/day6.in")?;
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut col_is_empty = vec![true; width];

    for x in 0..width {
        for line in &lines {
            if x < line.len() && line.as_bytes()[x] != b' ' {
                col_is_empty[x] = false;
                break;
            }
        }
    }

    let mut grand_total = 0;
    let mut block_start = None;

    for x in 0..width {
        let is_content = !col_is_empty[x];

        match (block_start, is_content) {
            (None, true) => {
                block_start = Some(x);
            }
            (Some(start), false) => {
                grand_total += process_block(&lines, start, x);
                block_start = None;
            }
            _ => {}
        }
    }

    if let Some(start) = block_start {
        grand_total += process_block(&lines, start, width);
    }

    Ok(grand_total)
}

fn process_block(lines: &[String], start: usize, end: usize) -> u64 {
    let mut numbers = Vec::new();
    let mut is_multiplication = false;

    for line in lines {
        if start >= line.len() {
            continue;
        }
        let slice_end = std::cmp::min(end, line.len());
        let part = line[start..slice_end].trim();

        if part.is_empty() {
            continue;
        }

        if part.contains('*') {
            is_multiplication = true;
        } else if part.contains('+') {
            is_multiplication = false;
        } else if let Ok(num) = part.parse::<u64>() {
            numbers.push(num);
        }
    }

    if is_multiplication {
        numbers.iter().product()
    } else {
        numbers.iter().sum()
    }
}
