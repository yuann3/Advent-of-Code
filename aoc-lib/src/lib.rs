use std::fs::read_to_string;
use std::io::{self};

pub fn read_lines(pathname: &str) -> io::Result<Vec<String>> {
    read_to_string(pathname).map(|content| {
        content
            .lines()
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect()
    })
}

pub fn read_to_char(pathname: &str) -> io::Result<Vec<char>> {
    read_to_string(pathname).map(|content| content.chars().collect())
}

pub fn read_single_line(pathname: &str) -> io::Result<String> {
    read_to_string(pathname).map(|content| content.trim().to_string())
}
