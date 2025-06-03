use anyhow::Result;
use aoc_lib::{self, read_lines};
use std::collections::HashMap;

enum Input {
    Wire(String),   // eg. "x", "y", "z"
    Value(u16),     // eg. 123456
}

enum Operation {
    Assign(Input),
    Not(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, u16),
    RShift(Input, u16),
}

struct Instruction {
    operation: Operation,
    output: String,
}

fn parse_input(s: &str) -> Input {
    match s.parse::<u16>() {
        Ok(number) => Input::Value(number),
        Err(_) => Input::Wire(s.to_string())
    }
}

fn parse_line(line: &str) -> Instruction {
    //println!("Parsing line: {}", line);
    // Split: "x AND y -> z" to ["x AND y", "z"]
    let parts: Vec<&str> = line.split(" -> ").collect();
    let operation_str = parts[0];
    let output = parts[1].to_string();

    //println!("Operation string: [{}]", operation_str);

    let operation = if operation_str.starts_with("NOT") {
        Operation::Not(parse_input(&operation_str[4..]))
    } else if operation_str.contains("AND") {
        let parts: Vec<&str> = operation_str.split(" AND ").collect();
        Operation::And(parse_input(parts[0]), parse_input(parts[1]))
    } else if operation_str.contains("OR") {
        let parts: Vec<&str> = operation_str.split(" OR ").collect();
        Operation::Or(parse_input(parts[0]), parse_input(parts[1]))
    } else if operation_str.contains("LSHIFT") {
        let parts: Vec<&str> = operation_str.split(" LSHIFT ").collect();
        Operation::LShift(parse_input(parts[0]), parts[1].parse().unwrap())
    } else if operation_str.contains("RSHIFT") {
        let parts: Vec<&str> = operation_str.split(" RSHIFT ").collect();
        Operation::RShift(parse_input(parts[0]), parts[1].parse().unwrap())
    } else {
        Operation::Assign(parse_input(operation_str))
    };

    Instruction {
        operation,
        output,
    }
}

fn evaluate_input(input: &Input, instructions: &[Instruction], cache: &mut HashMap<String, u16>) -> u16 {
    match input {
        Input::Value(v) => *v,
        Input::Wire(w) => evaluate_wire(w, instructions, cache),
    }
}

fn evaluate_wire(wire: &str, instructions: &[Instruction], cache: &mut HashMap<String, u16>) -> u16 {
    if let Some(&value) = cache.get(wire) {
        return value;
    }

    let instruction = instructions
        .iter()
        .find(|inst| inst.output == wire)
        .expect(&format!("No instruction found for wire: {}", wire));

    let result = match &instruction.operation {
        Operation::Assign(input) => {
            evaluate_input(input, instructions, cache)
        },
        Operation::Not(input) => {
            !evaluate_input(input, instructions, cache)  // Bitwise NOT
        },
        Operation::And(input1, input2) => {
            evaluate_input(input1, instructions, cache) & evaluate_input(input2, instructions, cache)
        },
        Operation::Or(input1, input2) => {
            evaluate_input(input1, instructions, cache) | evaluate_input(input2, instructions, cache)
        },
        Operation::LShift(input, shift) => {
            evaluate_input(input, instructions, cache) << shift
        },
        Operation::RShift(input, shift) => {
            evaluate_input(input, instructions, cache) >> shift
        },
    };

    cache.insert(wire.to_string(), result);
    result
}

// Main
pub fn solve() -> Result<u16> {
    let inputs = read_lines("input/day7.txt")?;
    let mut instructions: Vec<Instruction> = inputs
        .iter()
        .map(|line| parse_line(line))
        .collect();

    let mut wire_values: HashMap<String, u16> = HashMap::new();
    let a_value = evaluate_wire("a", &instructions, &mut wire_values);

    if let Some(b_instruction) = instructions.iter_mut().find(|inst| inst.output == "b") {
        b_instruction.operation = Operation::Assign(Input::Value(a_value));
    }

    wire_values.clear();
 
    let result = evaluate_wire("a", &instructions, &mut wire_values);

    Ok(result)
}
