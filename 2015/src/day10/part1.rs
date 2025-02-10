use aoc_lib::read_to_char;
use std::io;

pub fn solve() -> io::Result<usize> {
    let input: Vec<char> = read_to_char("input/day10.in")?
        .into_iter()
        .filter(|c| c.is_digit(10))
        .collect();
    println!("Starting input: {}", input.iter().collect::<String>());

    let mut sequence = input;

    for i in 0..5 {
        let old_sequence = sequence.clone();
        sequence = process_sequence(&sequence);
        let result = sequence.iter().collect::<String>();
        println!(
            "Iteration {}: {} -> {} (length: {})",
            i + 1,
            old_sequence.iter().collect::<String>(),
            result,
            result.len()
        );
    }

    for _ in 5..40 {
        sequence = process_sequence(&sequence);
    }

    Ok(sequence.len())
}

fn process_sequence(input: &[char]) -> Vec<char> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = input[0];
    let mut count = 1;

    for &digit in input.iter().skip(1) {
        if digit == current {
            count += 1;
        } else {
            result.push(char::from_digit(count as u32, 10).unwrap());
            result.push(current);
            current = digit;
            count = 1;
        }
    }

    result.push(char::from_digit(count as u32, 10).unwrap());
    result.push(current);

    result
}

#[test]
fn test_process_sequence() {
    assert_eq!(
        process_sequence(&vec!['1']).iter().collect::<String>(),
        "11"
    );
    assert_eq!(
        process_sequence(&vec!['1', '1']).iter().collect::<String>(),
        "21"
    );
    assert_eq!(
        process_sequence(&vec!['2', '1']).iter().collect::<String>(),
        "1211"
    );
}
