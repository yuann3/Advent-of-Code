use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day3.in")?;
    let mut total_output: u64 = 0;

    for line in lines {
        let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
        let n = digits.len();
        let k = 12;
        
        let mut result: u64 = 0;
        let mut start = 0;
        
        for remaining in (1..=k).rev() {
            let end = n - remaining;
            
            let mut max_digit = 0;
            let mut max_pos = start;
            for i in start..=end {
                if digits[i] > max_digit {
                    max_digit = digits[i];
                    max_pos = i;
                }
            }
            
            result = result * 10 + max_digit as u64;
            start = max_pos + 1;
        }
        total_output += result;
    }
    
    Ok(total_output)
}
