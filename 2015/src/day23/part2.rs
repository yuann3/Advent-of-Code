use anyhow::Result;
use aoc_lib::read_lines;

#[derive(Debug, Copy, Clone)]
enum Register {
    A,
    B,
}

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

#[derive(Debug)]
struct Computer {
    registers: [i32; 2],
    pc: usize,
    instructions: Vec<Instruction>,
}

impl Register {
    fn index(self) -> usize {
        match self {
            Register::A => 0,
            Register::B => 1,
        }
    }
}

impl Computer {
    fn new_with_initial(instructions: Vec<Instruction>, a: i32, b: i32) -> Self {
        Self {
            registers: [a, b],
            pc: 0,
            instructions,
        }
    }

    fn get_register(&self, reg: Register) -> i32 {
        self.registers[reg.index()]
    }

    fn set_register(&mut self, reg: Register, value: i32) {
        self.registers[reg.index()] = value;
    }

    fn run(&mut self) {
        while self.pc < self.instructions.len() {
            let instuction = &self.instructions[self.pc];

            match instuction {
                Instruction::Hlf(reg) => {
                    let value = self.get_register(*reg);
                    self.set_register(*reg, value / 2);
                    self.pc += 1;
                }
                Instruction::Tpl(reg) => {
                    let value = self.get_register(*reg);
                    self.set_register(*reg, value * 3);
                    self.pc += 1;
                }
                Instruction::Inc(reg) => {
                    let value = self.get_register(*reg);
                    self.set_register(*reg, value + 1);
                    self.pc += 1;
                }
                Instruction::Jmp(offset) => {
                    self.pc = (self.pc as i32 + offset) as usize;
                }
                Instruction::Jie(reg, offset) => {
                    let value = self.get_register(*reg);
                    if value % 2 == 0 {
                        self.pc = (self.pc as i32 + offset) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                Instruction::Jio(reg, offset) => {
                    let value = self.get_register(*reg);
                    if value == 1 {
                        self.pc = (self.pc as i32 + offset) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }
}

impl Instruction {
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();

        match parts[0] {
            "hlf" => {
                let reg = parse_registers(parts[1])?;
                Ok(Instruction::Hlf(reg))
            }
            "tpl" => {
                let reg = parse_registers(parts[1])?;
                Ok(Instruction::Tpl(reg))
            }
            "inc" => {
                let reg = parse_registers(parts[1])?;
                Ok(Instruction::Inc(reg))
            }
            "jmp" => {
                let offset = parts[1].parse::<i32>()?;
                Ok(Instruction::Jmp(offset))
            }
            "jie" => {
                let reg_str = parts[1].trim_end_matches(',');
                let reg = parse_registers(reg_str)?;
                let offset = parts[2].parse::<i32>()?;
                Ok(Instruction::Jie(reg, offset))
            }
            "jio" => {
                let reg_str = parts[1].trim_end_matches(',');
                let reg = parse_registers(reg_str)?;
                let offset = parts[2].parse::<i32>()?;
                Ok(Instruction::Jio(reg, offset))
            }
            _ => Err(anyhow::anyhow!("Unknown instruction: {}", parts[0])),
        }
    }
}

fn parse_registers(s: &str) -> Result<Register> {
    match s {
        "a" => Ok(Register::A),
        "b" => Ok(Register::B),
        _ => Err(anyhow::anyhow!("Invalid register: {}", s)),
    }
}

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day23.in")?;

    let instructions: Result<Vec<Instruction>> = lines
        .iter()
        .map(|line| Instruction::from_str(line))
        .collect();

    let mut computer = Computer::new_with_initial(instructions?, 1, 0);
    computer.run();

    Ok(computer.get_register(Register::B))
}
