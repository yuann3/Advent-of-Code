use anyhow::Result;
use aoc_lib::read_one_per_line;
use std::collections::HashSet;

pub fn solve() -> Result<u64> {
    let weights: Vec<u32> = read_one_per_line("input/day24.in")?;
    let total_weight: u32 = weights.iter().sum();

    if total_weight % 3 != 0 {
        return Err(anyhow::anyhow!("Total weight not divisible by 3"));
    }

    let target_weight = total_weight / 3;

    for group_size in 1..weights.len() {
        let mut combinations = Vec::new();
        let mut current = Vec::new();

        generate_combination(
            &weights,
            group_size,
            target_weight,
            0,
            0,
            &mut current,
            &mut combinations,
        );

        if !combinations.is_empty() {
            let valid_combinations: Vec<_> = combinations
                .into_iter()
                .filter(|combo| can_split(&weights, combo, target_weight))
                .collect();

            if !valid_combinations.is_empty() {
                let min_quantum_entanglement = valid_combinations
                    .iter()
                    .map(|combo| calculate_quantum_entanglement(&weights, combo))
                    .min()
                    .unwrap();

                return Ok(min_quantum_entanglement);
            }
        }
    }

    Ok(0)
}

fn calculate_quantum_entanglement(weights: &[u32], indices: &[usize]) -> u64 {
    indices.iter().map(|&i| weights[i] as u64).product()
}

fn can_split(weights: &[u32], use_indices: &[usize], target: u32) -> bool {
    let mut remaining = Vec::new();
    let used_set: HashSet<usize> = use_indices.iter().cloned().collect();

    for (i, &weight) in weights.iter().enumerate() {
        if !used_set.contains(&i) {
            remaining.push(weight);
        }
    }

    let remaining_sum: u32 = remaining.iter().sum();
    if remaining_sum != 2 * target {
        return false;
    }
    can_sum_to_target(&remaining, target)
}

fn can_sum_to_target(weights: &[u32], target: u32) -> bool {
    let mut dp = vec![false; (target + 1) as usize];
    dp[0] = true;

    for &weight in weights {
        for sum in (weight..=target).rev() {
            if dp[(sum - weight) as usize] {
                dp[sum as usize] = true;
            }
        }
    }

    dp[target as usize]
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

        current.pop();
    }
}
