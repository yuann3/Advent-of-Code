use anyhow::Result;
use aoc_lib::read_lines;
use std::collections::{HashMap, HashSet};

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashSet<String>,
) -> u64 {
    if current == target {
        return 1;
    }
    
    if visited.contains(current) {
        return 0;
    }
    
    visited.insert(current.to_string());
    
    let count = if let Some(neighbors) = graph.get(current) {
        neighbors
            .iter()
            .map(|next| count_paths(graph, next, target, visited))
            .sum()
    } else {
        0
    };
    
    visited.remove(current);
    count
}

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day11.in")?;
    
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in &lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let device = parts[0].to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(String::from).collect();
        graph.insert(device, outputs);
    }
    
    let mut visited = HashSet::new();
    let count = count_paths(&graph, "you", "out", &mut visited);
    
    Ok(count)
}
