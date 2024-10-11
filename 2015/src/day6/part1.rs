use aoc_lib::read_lines;
use std::error::Error;
use std::{collections::HashMap, usize};

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Command {
    instruction: Instruction,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_coordinate(string: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let coordinate: Vec<&str> = string.split(",").collect();
    if coordinate.len() != 2 {
        return Err("Invalid coordinate".into());
    }
    Ok((coordinate[0].parse()?, coordinate[1].parse()?))
}

fn parse_instructions(line: &str) -> Result<Command, Box<dyn Error>> {
    let words: Vec<&str> = line.split_whitespace().collect();

    let instruction = match words[0] {
        "toggle" => Instruction::Toggle,
        "turn" => match words[1] {
            "on" => Instruction::TurnOn,
            "off" => Instruction::TurnOff,
            _ => return Err("Invalid instruction".into()),
        },
        _ => return Err("Invalid instruction".into()),
    };

    let start_index = if matches!(instruction, Instruction::Toggle) {
        1
    } else {
        2
    };

    let start = parse_coordinate(words[start_index])?;
    let end = parse_coordinate(words.last().ok_or("Missing coordinates")?)?;

    Ok(Command {
        instruction,
        start,
        end,
    })
}

fn apply_instructions(light: &mut HashMap<(usize, usize), bool>, command: &Command) {
    for x in command.start.0..=command.end.0 {
        for y in command.start.1..=command.end.1 {
            match command.instruction {
                Instruction::TurnOn => {
                    light.insert((x, y), true);
                }
                Instruction::TurnOff => {
                    light.insert((x, y), false);
                }
                Instruction::Toggle => {
                    let entey = light.entry((x, y)).or_insert(false);
                    *entey = !*entey
                }
            }
        }
    }
}

fn count_lights(light: &HashMap<(usize, usize), bool>) -> usize {
    light.values().filter(|&&v| v).count()
}

pub fn solve() -> Result<usize, Box<dyn std::error::Error>> {
    let input = read_lines("input/day6p1.txt")?;
    let mut lights: HashMap<(usize, usize), bool> = HashMap::new();

    for line in input {
        let command = parse_instructions(&line)?;
        apply_instructions(&mut lights, &command);
    }

    Ok(count_lights(&lights))
}
