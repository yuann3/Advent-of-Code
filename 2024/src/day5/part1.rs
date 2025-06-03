use aoc_lib::read_lines_keep_empty;
use anyhow::{anyhow, Context, Result};

struct Rule {
    before: i32,
    after: i32,
}

pub fn solve() -> Result<i32> {
    let input = read_lines_keep_empty("input/day5.in")?;

    // debug
    //for (i, line) in input.iter().take(20).enumerate() {
    //    println!("Line {} : '{}'", i + 1, line);
    //}
    // debug

    let split_index = input.iter().position(|line| line.trim().is_empty())
        .context("Invalid input format: no empty line found to separate rules from updates")?;

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

fn parse_rules(lines: &[String]) -> Result<Vec<Rule>> {
    let mut rules = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let mut parts = line.split('|');
        let before = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .with_context(|| format!("Failed to parse 'before' value in rule on line {}", line_num + 1))?;
        let after = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .with_context(|| format!("Failed to parse 'after' value in rule on line {}", line_num + 1))?;

        rules.push(Rule { before, after });
    }

    Ok(rules)
}

fn parse_updates(lines: &[String]) -> Result<Vec<Vec<i32>>> {
    let mut updates = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let numbers: Result<Vec<i32>, _> = line
            .split(',')
            .enumerate()
            .map(|(num_index, s)| {
                s.trim().parse::<i32>()
                    .with_context(|| format!("Failed to parse number '{}' (position {}) on update line {}", s.trim(), num_index + 1, line_num + 1))
            })
            .collect();

        let numbers = numbers?;

        if numbers.is_empty() {
            return Err(anyhow!("Empty update on line {}", line_num + 1));
        }
        updates.push(numbers);
    }

    Ok(updates)
}
