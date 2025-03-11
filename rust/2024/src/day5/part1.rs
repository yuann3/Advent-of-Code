use aoc_lib::read_lines_keep_empty;
use std::io;

struct Rule {
    before: i32,
    after: i32,
}

pub fn solve() -> io::Result<i32> {
    let input = read_lines_keep_empty("input/day5.in")?;

    // debug
    //for (i, line) in input.iter().take(20).enumerate() {
    //    println!("Line {} : '{}'", i + 1, line);
    //}
    // debug

    let split_index = match input.iter().position(|line| line.trim().is_empty()) {
        Some(index) => index,
        None => {
            println!("no empty line");
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid input format",
            ));
        }
    };

    let (rules_part, updates_part) = input.split_at(split_index);
    let updates_part = &updates_part[1..];

    let rules = parse_rules(rules_part)?;
    let updates = parse_updates(updates_part)?;

    let sum: i32 = updates
        .iter()
        .filter(|update| is_valid_order(update, &rules))
        .map(|update| get_middle_num(update))
        .sum();

    Ok(sum)
}

fn get_middle_num(update: &[i32]) -> i32 {
    update[update.len() / 2]
}

fn is_valid_order(update: &[i32], rules: &[Rule]) -> bool {
    for rule in rules {
        let before_pos = update.iter().position(|&x| x == rule.before);
        let after_pos = update.iter().position(|&x| x == rule.after);

        if let (Some(before_idx), Some(after_idx)) = (before_pos, after_pos) {
            if before_idx > after_idx {
                return false;
            }
        }
    }
    true
}

fn parse_rules(lines: &[String]) -> io::Result<Vec<Rule>> {
    let mut rules = Vec::new();

    for line in lines {
        let mut parts = line.split('|');
        let before = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid rule format"))?;
        let after = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid rule format"))?;

        rules.push(Rule { before, after });
    }

    Ok(rules)
}

fn parse_updates(lines: &[String]) -> io::Result<Vec<Vec<i32>>> {
    let mut updates = Vec::new();

    for line in lines {
        let numbers: Vec<i32> = line
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if numbers.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Empty update line",
            ));
        }
        updates.push(numbers);
    }

    Ok(updates)
}
