use aoc_lib::read_lines;
use anyhow::{Context, Result};

struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn check_combinations(&self) -> bool {
    fn try_combinations(
        numbers: &[i64],
            operators: &mut Vec<u8>,
        pos: usize,
        target: i64,
    ) -> bool {
        if pos == operators.len() {
            let result = evaluate(numbers, operators);
            return result == target;
        }

            for op in 0..3 {
                operators[pos] = op;
                if try_combinations(numbers, operators, pos + 1, target) {
            return true;
        }
            }
            false
        }

        let mut operators = vec![0; self.numbers.len() - 1];
        try_combinations(&self.numbers, &mut operators, 0, self.test_value)
    }
}

fn concatenate(a: i64, b: i64) -> i64 {
    let b_str = b.to_string();
    let concatenated = format!("{}{}", a, b_str);
    concatenated.parse().unwrap_or(0)
}

fn evaluate(numbers: &[i64], operators: &[u8]) -> i64 {
    let mut result = numbers[0];

    for i in 0..operators.len() {
        match operators[i] {
            0 => result += numbers[i + 1],       // +
            1 => result *= numbers[i + 1],       // *
            2 => result = concatenate(result, numbers[i + 1]), // ||
            _ => panic!("Invalid operator"),
        }
    }

    result
}

pub fn solve() -> Result<i64> {
    let input = read_lines("input/day7.in")?;
    let mut sum = 0;

    for (line_num, line) in input.iter().enumerate() {
        let parts: Vec<&str> = line.split(':').collect();
        
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid format on line {}: expected 'value: numbers'", line_num + 1));
        }
        
        let test_value = parts[0].trim().parse::<i64>()
            .with_context(|| format!("Failed to parse test value '{}' on line {}", parts[0].trim(), line_num + 1))?;
        
        let numbers: Result<Vec<i64>, _> = parts[1]
            .split_whitespace()
            .enumerate()
            .map(|(num_index, n)| {
                n.parse::<i64>()
                    .with_context(|| format!("Failed to parse number '{}' (position {}) on line {}", n, num_index + 1, line_num + 1))
            })
            .collect();
        
        let numbers = numbers?;

        let equation = Equation {
            test_value,
            numbers,
        };

        if equation.check_combinations() {
            sum += equation.test_value;
        }
    }

    Ok(sum)
}
