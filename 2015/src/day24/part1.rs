use anyhow::Result;
use aoc_lib::read_one_per_line;

pub fn solve() -> Result<u32> {
    let weights = read_one_per_line("input/day24.in")?;
    let total_weigth: u32 = weights.iter().sum();
    let target_weight = total_weigth / 3;

    for group_size in 1..weights.len() {
        let mut combinations = Vec::new();
        let mut current = Vec::new();

        generate_combination(
            &weights,
            size,
            target_weight,
            0,
            0,
            &mut current,
            &mut combinations,
        );

        if !combinations.is_empty() {
            // TODO: Next step - validation and quantum entanglement
            break;
        }
    }

    Ok(target_weight)
}

fn generate_combination(
    weights: &[u32],
    size: usize,
    target: u32,
    start: usize,
    current_sum: u32,
    current: &mut Vec<usize>,
    results: &mut Vec<Vec<usize>>,
) {
    if current_sum > target {
        return;
    }

    if current.len() == size {
        if current_sum == target {
            results.push(current.clone());
        }
    }

    if current.len() + (weights.len() - start) < size {
        return;
    }

    for i in start..weights.len() {
        current.push(i);
        generate_combination(
            weights,
            size,
            target,
            i + 1,
            current_sum + weights[i],
            current,
            results,
        );

        current.pop()
    }
}
