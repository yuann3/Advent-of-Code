use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

pub mod benchmark;
pub use benchmark::*;

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
    let path_ref = path.as_ref();
    let content = read_to_string(path_ref)
        .with_context(|| format!("Failed to read file: {}", path_ref.display()))?;
    
    Ok(content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
        .collect())
}

/// Reads a file and returns all lines, including empty ones, as a vector of strings.
///
/// This function is particularly useful when working with files that use empty lines
/// as meaningful separators between sections of content. Unlike `read_lines`, which
/// filters out empty lines, this function preserves them.
///
/// # Arguments
///
/// * `path` - Any value that can be converted into a Path (like &str or String).
///   The path is relative to the current working directory.
///
/// # Returns
///
/// * `Result<Vec<String>>` - If successful, returns a vector where each element is
///   a line from the file (including empty lines). If an error occurs (e.g., file
///   not found or permission denied), returns an IO error.
///
/// # Examples
///
/// ```
/// use aoc_lib::read_lines_keep_empty;
///
/// // Reading a file with section separators
/// let lines = read_lines_keep_empty("input.txt").unwrap();
/// // If input.txt contains:
/// // Section 1
/// //
/// // Section 2
/// // The vector will contain 3 elements, including the empty line
/// ```
///
/// # Technical Details
///
/// The function uses `read_to_string` internally to read the entire file into memory,
/// then splits it into lines. Each line is converted to a String, preserving empty
/// lines. The newline characters themselves are not included in the resulting strings.
pub fn read_lines_keep_empty<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let path_ref = path.as_ref();
    Ok(read_to_string(path_ref)
        .with_context(|| format!("Failed to read file: {}", path_ref.display()))?
        .lines()
        .map(String::from)
        .collect())
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
    let path_ref = path.as_ref();
    Ok(read_to_string(path_ref)
        .with_context(|| format!("Failed to read file: {}", path_ref.display()))?
        .chars()
        .collect())
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
    let path_ref = path.as_ref();
    let content = read_to_string(path_ref)
        .with_context(|| format!("Failed to read file: {}", path_ref.display()))?;
    
    let first_line = content
        .lines()
        .next()
        .with_context(|| format!("File is empty: {}", path_ref.display()))?;
    
    let parsed_values: Result<Vec<T>, _> = first_line
        .split(sep)
        .map(|part| {
            part.trim().parse::<T>()
                .map_err(|e| anyhow::anyhow!("Failed to parse '{}': {:?}", part.trim(), e))
        })
        .collect();
    
    parsed_values.with_context(|| format!("Failed to parse line from file: {}", path_ref.display()))
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
    let path_ref = path.as_ref();
    let content = read_to_string(path_ref)
        .with_context(|| format!("Failed to read file: {}", path_ref.display()))?;
    
    let parsed_values: Result<Vec<T>, _> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            line.parse::<T>()
                .map_err(|e| anyhow::anyhow!("Failed to parse line {}: '{}': {:?}", line_num + 1, line, e))
        })
        .collect();
    
    parsed_values.with_context(|| format!("Failed to parse lines from file: {}", path_ref.display()))
}
