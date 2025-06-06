use anyhow::Result;
use aoc_lib::read_lines;
use std::collections::{HashMap, HashSet};

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day13.in")?;

    let mut happiness = HashMap::new();
    let mut people = HashSet::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let person1 = parts[0];
        let gain_lose = parts[2];
        let amount: i32 = parts[3].parse()?;
        let person2 = parts[10].trim_end_matches('.');

        people.insert(person1.to_string());
        people.insert(person2.to_string());

        let happiness_value = if gain_lose == "gain" { amount } else { -amount };

        happiness
            .entry(person1.to_string())
            .or_insert_with(HashMap::new)
            .insert(person2.to_string(), happiness_value);
    }

    // part 2
    let me = "Me".to_string();
    people.insert(me.clone());

    let people_list: Vec<String> = people.iter().cloned().collect();

    for person in &people_list {
        if person != &me{
            happiness.entry(me.clone())
                     .or_insert_with(HashMap::new)
                     .insert(person.clone(), 0);

            happiness.entry(person.clone())
                     .or_insert_with(HashMap::new)
                     .insert(me.clone(), 0);
        }
    }

    let people_vec: Vec<String> = people.into_iter().collect();
    let max_happiness = find_optimal_seating(&people_vec, &happiness);

    Ok(max_happiness)
}

fn find_optimal_seating(
    people: &[String],
    happiness: &HashMap<String, HashMap<String, i32>>,
) -> i32 {
    if people.is_empty() {
        return 0;
    }

    let first_person = &people[0];
    let remaining: Vec<String> = people[1..].to_vec();
    let mut max_happiness = i32::MIN ;

    for perm in permutations(&remaining) {
        let mut arrangement = vec![first_person.clone()];
        arrangement.extend(perm);

        let total_happiness = calculate_happiness(&arrangement, happiness);
        max_happiness = max_happiness.max(total_happiness);
    }

    max_happiness
}

fn calculate_happiness(
    arrangement: &[String],
    happiness: &HashMap<String, HashMap<String, i32>>,
) -> i32 {
    let n = arrangement.len();
    let mut total = 0;

    for i in 0..n {
        let person = &arrangement[i];
        let left_neighbor = &arrangement[(i + n - 1) % n];
        let right_neighbor = &arrangement[(i + 1) % n];

        if let Some(person_happiness) = happiness.get(person) {
            if let Some(&left_happiness) = person_happiness.get(left_neighbor) {
                total += left_happiness;
            }
            if let Some(&right_happiness) = person_happiness.get(right_neighbor) {
                total += right_happiness;
            }
        }
    }

    total
}

fn permutations(items: &[String]) -> Vec<Vec<String>> {
    if items.len() < 1 {
        return vec![items.to_vec()];
    }

    let mut result = Vec::new();
    for i in 0..items.len() {
        let mut remaining = items.to_vec();
        let current = remaining.remove(i);

        for mut perm in permutations(&remaining) {
            perm.insert(0, current.clone());
            result.push(perm);
        }
    }

    result
}
