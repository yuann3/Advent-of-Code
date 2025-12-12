use anyhow::Result;
use aoc_lib::read_lines;
use std::collections::HashMap;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day11.in")?;

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in &lines {
        let (src, dests) = line.split_once(": ").unwrap();
        graph.insert(src, dests.split_whitespace().collect());
    }

    let mut memo: HashMap<(&str, bool, bool), u64> = HashMap::new();
    let count = count_paths(&graph, "svr", false, false, &mut memo);

    Ok(count)
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    seen_dac: bool,
    seen_fft: bool,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    let seen_dac = seen_dac || node == "dac";
    let seen_fft = seen_fft || node == "fft";

    if node == "out" {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }

    let key = (node, seen_dac, seen_fft);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let Some(neighbors) = graph.get(node) else {
        return 0;
    };

    let result = neighbors
        .iter()
        .map(|next| count_paths(graph, next, seen_dac, seen_fft, memo))
        .sum();

    memo.insert(key, result);
    result
}
