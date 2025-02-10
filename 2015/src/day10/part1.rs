use aoc_lib::read_to_char;
use std::io;

pub fn solve() -> io::Result<usize> {
    let input: Vec<char> = read_to_char("input/day10.in")?;

    let mut sequence = input;

    for _ in 0..40 {
        sequence = process_sequence(&sequence);
    }

    Ok(sequence.len())
}

fn process_sequence(input: &[char]) -> Vec<char> {
    let mut result = Vec::new();
    let mut count = 0;
    let mut current_digit = None;

    for &digit in input {
        match current_digit {
            Some(current) => {
                if current == digit {
                    count += 1;
                } else {
                    result.push(char::from_digit(count as u32, 10).unwrap());
                    result.push(current);

                    current_digit = Some(digit);
                    count = 1;
                }
            }
            None => {
                current_digit = Some(digit);
                count = 1;
            }
        }
    }

    if let Some(last_digit) = current_digit {
        result.push(char::from_digit(count as u32, 10).unwrap());
        result.push(last_digit);
    }

    result
}
