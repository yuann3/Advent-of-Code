use aoc_lib::read_lines;
use std::io;

struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

#[derive(Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Equation {
    fn check_combinations(&self) -> bool {
        let mut operators = vec![Operator::Add; self.numbers.len() - 1];
        Self::try_combinations(&self.numbers, &mut operators, 0, self.test_value)
    }

    fn try_combinations(
        numbers: &[i64],
        operators: &mut Vec<Operator>,
        pos: usize,
        target: i64,
    ) -> bool {
        if pos == operators.len() {
            let result = evaluate(numbers, operators);
            return result == target;
        }

        operators[pos] = Operator::Add;
        if Self::try_combinations(numbers, operators, pos + 1, target) {
            return true;
        }

        operators[pos] = Operator::Multiply;
        if Self::try_combinations(numbers, operators, pos + 1, target) {
            return true;
        }

        operators[pos] = Operator::Concat;
        Self::try_combinations(numbers, operators, pos + 1, target)
    }
}

fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];

    for i in 0..operators.len() {
        match operators[i] {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
            Operator::Concat => {
                let concat = format!("{}{}", result, numbers[i + 1]);
                result = concat.parse().unwrap();
            }
        }
    }

    result
}

pub fn solve() -> io::Result<i64> {
    let input = read_lines("input/day7.in")?;
    let mut sum = 0;

    for line in input {
        let parts: Vec<&str> = line.split(':').collect();
        let test_value = parts[0].trim().parse::<i64>().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

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
