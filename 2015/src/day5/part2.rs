use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

pub fn is_nice(s: &str) -> bool {
    has_pair_appearing_twice(s) && has_repeating_letter_with_one_between(s)
}

fn has_pair_appearing_twice(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        let pair = format!("{}{}", chars[i], chars[i + 1]);
        if s[i + 2..].contains(&pair) {
            return true;
        }
    }
    false
}

fn has_repeating_letter_with_one_between(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

fn count_nice_strings(strings: &[String]) -> usize {
    strings.iter().filter(|s| is_nice(s)).count()
}

pub fn read_lines<T>(filename: T) -> Result<Vec<String>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect::<std::io::Result<Vec<String>>>().map_err(Into::into)
}

pub fn solve() -> Result<usize> {
    let lines = read_lines("input/day5p1.txt")?;
    Ok(count_nice_strings(&lines))
}
