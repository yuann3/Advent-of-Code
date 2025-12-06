use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day6.in")?;
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    
    let max_width = grid.iter().map(|row| row.len()).max().unwrap_or(0);
    let height = grid.len();

    let get = |r: usize, c: usize| -> char {
        grid.get(r).and_then(|row| row.get(c)).copied().unwrap_or(' ')
    };

    let mut total: u64 = 0;
    let mut nums: Vec<u64> = vec![];
    let mut op = '+';

    for col in (0..max_width).rev() {
        let chars: Vec<char> = (0..height).map(|r| get(r, col)).collect();

        if chars.iter().all(|&c| c == ' ') {
            if !nums.is_empty() {
                let result = match op {
                    '+' => nums.iter().sum(),
                    '*' => nums.iter().product(),
                    _ => 0,
                };
                total += result;
                nums.clear();
            }
            continue;
        }

        let last = chars[height - 1];
        if last == '+' || last == '*' {
            op = last;
        }

        let num_str: String = chars[..height - 1]
            .iter()
            .filter(|c| c.is_ascii_digit())
            .collect();

        if !num_str.is_empty() {
            nums.push(num_str.parse().unwrap());
        }
    }

    if !nums.is_empty() {
        let result = match op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => 0,
        };
        total += result;
    }

    Ok(total)
}
