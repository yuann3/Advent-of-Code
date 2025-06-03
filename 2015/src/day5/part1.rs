use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

pub fn is_nice(s: &str) -> bool {
    has_three_vowels(s) && has_double_letter(s) && !has_forbidden_substrings(s)
}

fn has_three_vowels(s: &str) -> bool {
    s.chars()
        .filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .take(3)
        .count()
        == 3
}

fn has_double_letter(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
}

fn has_forbidden_substrings(s: &str) -> bool {
    ["ab", "cd", "pq", "xy"].iter().any(|&sub| s.contains(sub))
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
