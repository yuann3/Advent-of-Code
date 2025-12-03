use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u32> {
    let lines = read_lines("input/day3.in")?;
    let mut total_output = 0;
    for line in lines {
        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        'tens_loop: for tens in (1..=9u32).rev() {
            if let Some(first_idx) = digits.iter().position(|&d| d == tens) {
                if first_idx < digits.len() - 1 {
                    let max_ones = digits[first_idx + 1..]
                        .iter()
                        .max()
                        .unwrap();
                    
                    total_output += tens * 10 + max_ones;
                    break 'tens_loop;
                }
            }
        }
    }

    Ok(total_output)
}
