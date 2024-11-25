use aoc_lib::read_single_line;
use std::{io, usize};

pub fn solve() -> io::Result<i32> {
    let input: Vec<i32> = read_single_line("input/day2.txt", ",").unwrap();

    let mut program = input.clone();

    program[1] = 12;
    program[2] = 2;

    let mut position = 0;

    while position < program.len() {
        match program[position] {
            99 => break,
            1 => {
                let pos1 = program[position + 1] as usize;
                let pos2 = program[position + 2] as usize;
                let pos3 = program[position + 3] as usize;
                program[pos3] = program[pos1] + program[pos2];
            },
            2 => {
                let pos1 = program[position + 1] as usize;
                let pos2 = program[position + 2] as usize;
                let pos3 = program[position + 3] as usize;
                program[pos3] = program[pos1] * program[pos2];
            },
            _ => panic!("Unknow opcode: {}", program[position]),
        }
        position += 4;
    }
    Ok(program[0])
}
