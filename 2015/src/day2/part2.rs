use crate::day2::part1::{parse_line, Prism};
use aoc_lib::read_lines;
use std::io;

fn ribbon_length(prism: &Prism) -> i32 {
    let mut dimensions = [prism.length, prism.width, prism.height];
    dimensions.sort_unstable();

    let perimeter = 2 * (dimensions[0] + dimensions[1]);
    let volume = prism.length * prism.width * prism.height;

    perimeter + volume
}

pub fn solve() -> io::Result<i32> {
    let input = read_lines("input/day2p1.txt")?;

    input
        .iter()
        .enumerate()
        .try_fold(0, |acc, (line_number, line)| {
            parse_line(line)
                .map(|prism| acc + ribbon_length(&prism))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Error in line {}: {}", line_number + 1, line),
                    )
                })
        })
}
