use std::fs::read_to_string;
use std::io::{self, Result};
use std::path::Path;
use std::str::FromStr;

/// Reads lines from a file and returns a vector of non-empty strings.
///
/// # Arguments
///
/// * `path` - A value that can be converted into a Path
///
/// # Returns
///
/// * `Result<Vec<String>>` - A Result containing a vector of Strings, each representing a non-empty line from the file
///
/// # Examples
///
/// ```
/// use aoc_lib::read_lines;
///
/// let lines = read_lines("example.txt").unwrap();
/// println!("Number of non-empty lines: {}", lines.len());
/// ```
pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    read_to_string(path).map(|content| {
        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect()
    })
}

/// Reads a file and returns its content as a vector of characters.
///
/// # Arguments
///
/// * `path` - A value that can be converted into a Path
///
/// # Returns
///
/// * `Result<Vec<char>>` - A Result containing a vector of chars, representing all characters in the file
///
/// # Examples
///
/// ```
/// use aoc_lib::read_to_char;
///
/// let chars = read_to_char("example.txt").unwrap();
/// println!("Number of characters: {}", chars.len());
/// ```
pub fn read_to_char<P: AsRef<Path>>(path: P) -> Result<Vec<char>> {
    read_to_string(path).map(|content| content.chars().collect())
}

/// Reads a single line from a file, splits it by a separator, and parses each part into a value of type T.
///
/// # Arguments
///
/// * `path` - A value that can be converted into a Path
/// * `sep` - A string slice used as the separator
///
/// # Type Parameters
///
/// * `T` - The type to parse each part into. Must implement `FromStr` trait.
///
/// # Returns
///
/// * `Result<Vec<T>>` - A Result containing a vector of successfully parsed values
///
/// # Examples
///
/// ```
/// use aoc_lib::read_single_line;
///
/// let numbers: Vec<i32> = read_single_line("numbers.txt", ",").unwrap();
/// println!("Numbers: {:?}", numbers);
/// ```
pub fn read_single_line<P, T>(path: P, sep: &str) -> Result<Vec<T>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let content = read_to_string(path)?;
    Ok(content
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "File is empty"))?
        .split(sep)
        .filter_map(|part| part.trim().parse().ok())
        .collect())
}

/// Reads a file and parses each non-empty line into a value of type T.
///
/// # Arguments
///
/// * `path` - A value that can be converted into a Path
///
/// # Type Parameters
///
/// * `T` - The type to parse each line into. Must implement `FromStr` trait.
///
/// # Returns
///
/// * `Result<Vec<T>>` - A Result containing a vector of successfully parsed values
///
/// # Examples
///
/// ```
/// use aoc_lib::read_one_per_line;
///
/// let numbers: Vec<i32> = read_one_per_line("numbers.txt").unwrap();
/// println!("Numbers: {:?}", numbers);
/// ```
pub fn read_one_per_line<P, T>(path: P) -> Result<Vec<T>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    read_to_string(path).map(|content| {
        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| line.parse().ok())
            .collect()
    })
}
