use anyhow::{Context, Result};
use aoc_lib::read_lines;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
enum Dest {
    Bot(u32),
    Output(u32),
}

pub fn solve() -> Result<u32> {
    let lines = read_lines("input/day10.in")?;

    let mut instructions: HashMap<u32, (Dest, Dest)> = HashMap::new();
    let mut initials: Vec<(u32, u32)> = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "value" {
            let chip: u32 = words[1].parse().context("Parse chip")?;
            let bot: u32 = words[5].parse().context("Parse bot")?;
            initials.push((chip, bot));
        } else if words[0] == "bot" {
            let bot_id: u32 = words[1].parse().context("Parse bot_id")?;
            let low_type = words[5];
            let low_id: u32 = words[6].parse().context("Parse low_id")?;
            let high_type = words[10];
            let high_id: u32 = words[11].parse().context("Parse high_id")?;
            let low_dest = if low_type == "bot" {
                Dest::Bot(low_id)
            } else {
                Dest::Output(low_id)
            };
            let high_dest = if high_type == "bot" {
                Dest::Bot(high_id)
            } else {
                Dest::Output(high_id)
            };
            instructions.insert(bot_id, (low_dest, high_dest));
        }
    }

    let mut bots: HashMap<u32, Vec<u32>> = HashMap::new();
    for (chip, bot) in initials {
        bots.entry(bot).or_insert(Vec::new()).push(chip);
    }

    let mut queue: VecDeque<u32> = VecDeque::new();
    for (&bot_id, chips) in &bots {
        if chips.len() == 2 {
            queue.push_back(bot_id);
        }
    }

    let mut found: Option<u32> = None;

    while let Some(bot_id) = queue.pop_front() {
        let instr = *instructions.get(&bot_id).context("No instr")?;
        let chips = bots.get_mut(&bot_id).context("No chips")?;
        chips.sort();
        let low = chips.remove(0);
        let high = chips.remove(0);

        if low == 17 && high == 61 {
            found = Some(bot_id);
        }

        match instr.0 {
            Dest::Bot(id) => {
                let target = bots.entry(id).or_insert(Vec::new());
                target.push(low);
                if target.len() == 2 {
                    queue.push_back(id);
                }
            }
            Dest::Output(_) => {}
        }

        match instr.1 {
            Dest::Bot(id) => {
                let target = bots.entry(id).or_insert(Vec::new());
                target.push(high);
                if target.len() == 2 {
                    queue.push_back(id);
                }
            }
            Dest::Output(_) => {}
        }
    }

    found.context("No bot found comparing 17 and 61")
}
