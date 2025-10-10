use anyhow::Result;
use aoc_lib::read_lines;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug)]
enum Dest {
    Bot(usize),
    Output(usize),
}

struct BotInstr {
    low: Dest,
    high: Dest,
}

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day10.in")?;

    let mut instructions: HashMap<usize, BotInstr> = HashMap::new();
    let mut initial: Vec<(u32, usize)> = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "value" {
            let val: u32 = words[1].parse()?;
            let bot: usize = words[5].parse()?;
            initial.push((val, bot));
        } else if words[0] == "bot" {
            let bot: usize = words[1].parse()?;
            let low_type = words[5];
            let low_id: usize = words[6].parse()?;
            let high_type = words[10];
            let high_id: usize = words[11].parse()?;
            let low = if low_type == "bot" {
                Dest::Bot(low_id)
            } else {
                Dest::Output(low_id)
            };
            let high = if high_type == "bot" {
                Dest::Bot(high_id)
            } else {
                Dest::Output(high_id)
            };
            instructions.insert(bot, BotInstr { low, high });
        }
    }

    let mut bot_chips: HashMap<usize, Vec<u32>> = HashMap::new();
    let mut outputs: HashMap<usize, Vec<u32>> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new();

    for (val, bot) in initial {
        let chips = bot_chips.entry(bot).or_insert_with(Vec::new);
        chips.push(val);
        if chips.len() == 2 {
            queue.push_back(bot);
        }
    }

    while let Some(bot) = queue.pop_front() {
        let opt_chips = bot_chips.remove(&bot);
        if let Some(mut chips) = opt_chips {
            if chips.len() != 2 {
                bot_chips.insert(bot, chips);
                continue;
            }
            chips.sort();
            let low_val = chips[0];
            let high_val = chips[1];

            let instr = instructions.get(&bot).unwrap();

            match instr.low {
                Dest::Bot(b) => {
                    let target_chips = bot_chips.entry(b).or_insert_with(Vec::new);
                    target_chips.push(low_val);
                    if target_chips.len() == 2 {
                        queue.push_back(b);
                    }
                }
                Dest::Output(o) => {
                    outputs.entry(o).or_insert_with(Vec::new).push(low_val);
                }
            }

            match instr.high {
                Dest::Bot(b) => {
                    let target_chips = bot_chips.entry(b).or_insert_with(Vec::new);
                    target_chips.push(high_val);
                    if target_chips.len() == 2 {
                        queue.push_back(b);
                    }
                }
                Dest::Output(o) => {
                    outputs.entry(o).or_insert_with(Vec::new).push(high_val);
                }
            }
        }
    }

    let out0 = outputs
        .get(&0)
        .and_then(|v| v.first())
        .copied()
        .unwrap_or(0) as i32;
    let out1 = outputs
        .get(&1)
        .and_then(|v| v.first())
        .copied()
        .unwrap_or(0) as i32;
    let out2 = outputs
        .get(&2)
        .and_then(|v| v.first())
        .copied()
        .unwrap_or(0) as i32;

    Ok(out0 * out1 * out2)
}
