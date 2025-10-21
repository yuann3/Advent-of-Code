use anyhow::{anyhow, Result};
use aoc_lib::read_lines;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone)]
enum Item {
    Gen(usize),
    Chip(usize),
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    elevator: u8,
    positions: Vec<(u8, u8)>,
}

fn combinations(items: &[Item], k: usize) -> Vec<Vec<Item>> {
    let mut res: Vec<Vec<Item>> = Vec::new();
    let mut curr: Vec<Item> = Vec::new();
    comb_helper(items, k, 0, &mut curr, &mut res);
    res
}

fn comb_helper(
    items: &[Item],
    k: usize,
    start: usize,
    curr: &mut Vec<Item>,
    res: &mut Vec<Vec<Item>>,
) {
    if curr.len() == k {
        res.push(curr.clone());
        return;
    }
    for i in start..items.len() {
        curr.push(items[i]);
        comb_helper(items, k, i + 1, curr, res);
        curr.pop();
    }
}

fn valid_floor(pos: &[(u8, u8)], f: u8) -> bool {
    let mut gens = 0;
    let mut unprotected = 0;
    for &(g, m) in pos {
        if g == f {
            gens += 1;
        }
        if m == f && g != f {
            unprotected += 1;
        }
    }
    gens == 0 || unprotected == 0
}

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day11.in")?;

    let mut element_to_pos: HashMap<String, (Option<u8>, Option<u8>)> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let floor = i as u8;
        let content = line.split(" contains ").nth(1).unwrap_or("");
        let items_str = content.replace("and ", ", ").replace(".", "");
        let items: Vec<&str> = items_str
            .split(", ")
            .filter(|s| !s.is_empty() && *s != "nothing relevant")
            .collect();
        for item in items {
            if item.ends_with(" generator") {
                let elem = item
                    .strip_suffix(" generator")
                    .unwrap()
                    .strip_prefix("a ")
                    .unwrap()
                    .to_string();
                element_to_pos.entry(elem).or_insert((None, None)).0 = Some(floor);
            } else if item.ends_with("-compatible microchip") {
                let elem = item
                    .strip_suffix("-compatible microchip")
                    .unwrap()
                    .strip_prefix("a ")
                    .unwrap()
                    .to_string();
                element_to_pos.entry(elem).or_insert((None, None)).1 = Some(floor);
            }
        }
    }

    element_to_pos.insert("elerium".to_string(), (Some(0), Some(0)));
    element_to_pos.insert("dilithium".to_string(), (Some(0), Some(0)));

    let mut positions: Vec<(u8, u8)> = element_to_pos
        .values()
        .map(|&(g, m)| (g.unwrap(), m.unwrap()))
        .collect();
    positions.sort();

    let initial = State {
        elevator: 0,
        positions,
    };

    let mut queue: VecDeque<(State, i32)> = VecDeque::new();
    queue.push_back((initial, 0));

    let mut visited: HashSet<State> = HashSet::new();
    visited.insert(queue.front().unwrap().0.clone());

    while let Some((state, steps)) = queue.pop_front() {
        if state.positions.iter().all(|&(g, m)| g == 3 && m == 3) {
            return Ok(steps);
        }

        let e = state.elevator;
        let dirs: Vec<i8> = if e == 0 {
            vec![1]
        } else if e == 3 {
            vec![-1]
        } else {
            vec![1, -1]
        };

        for d in dirs {
            let target = (e as i8 + d) as u8;

            let mut items: Vec<Item> = vec![];
            for (i, &(g, m)) in state.positions.iter().enumerate() {
                if g == e {
                    items.push(Item::Gen(i));
                }
                if m == e {
                    items.push(Item::Chip(i));
                }
            }

            for take in 1..=2 {
                if take > items.len() {
                    continue;
                }
                let combos = combinations(&items, take);
                for combo in combos {
                    let mut new_pos = state.positions.clone();
                    for it in combo {
                        match it {
                            Item::Gen(pid) => new_pos[pid].0 = target,
                            Item::Chip(pid) => new_pos[pid].1 = target,
                        }
                    }

                    if !valid_floor(&new_pos, e) || !valid_floor(&new_pos, target) {
                        continue;
                    }

                    new_pos.sort();
                    let new_state = State {
                        elevator: target,
                        positions: new_pos,
                    };

                    if visited.insert(new_state.clone()) {
                        queue.push_back((new_state, steps + 1));
                    }
                }
            }
        }
    }

    Err(anyhow!("No solution found"))
}
