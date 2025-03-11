use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn is_nice(s: &str) -> bool {
    has_repeating_pair(s) && has_repeating_letter_with_one_between(s)
}

fn has_repeating_pair(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut pairs = HashMap::new();

    for i in 0..chars.len() - 1 {
        let pair = (chars[i], chars[i + 1]);
        if let Some(&prev_index) = pairs.get(&pair) {
            if i > prev_index + 1 {
                return true;
            }
        } else {
            pairs.insert(pair, i);
        }
    }
    false
}

fn has_repeating_letter_with_one_between(s: &str) -> bool {
    s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b)
}

fn count_nice_strings(strings: &[String]) -> usize {
    strings.iter().filter(|s| is_nice(s)).count()
}

pub fn read_lines<T>(filename: T) -> std::io::Result<Vec<String>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn solve() -> std::io::Result<usize> {
    let lines = read_lines("input/day5p1.txt")?;
    Ok(count_nice_strings(&lines))
}
