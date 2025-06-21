use anyhow::Result;
use aoc_lib::read_one_per_line;
use std::collections::HashSet;

pub fn solve() -> Result<u64> {
    solve_for_n_groups(4)
}

fn solve_for_n_groups(num_groups: u32) -> Result<u64> {
    let weights: Vec<u32> = read_one_per_line("input/day24.in")?;
    let total_weight: u32 = weights.iter().sum();

    if total_weight % num_groups != 0 {
        return Err(anyhow::anyhow!(
            "Total weight {} is not divisible by {}",
            total_weight,
            num_groups
        ));
    }

    let target_weight = total_weight / num_groups;

    for group_size in 1..weights.len() {
        let mut combinations = Vec::new();
        let mut current = Vec::new();

        generate_combinations_with_sum(
            &weights,
            group_size,
            target_weight,
            0,
            0,
            &mut current,
            &mut combinations,
        );

        if combinations.is_empty() {
            continue;
        }

        let valid_combinations: Vec<_> = combinations
            .into_iter()
            .filter(|combo| {
                can_partition_remaining(
                    &weights,
                    combo,
                    target_weight,
                    num_groups - 1,
                )
            })
            .collect();

        if !valid_combinations.is_empty() {
            return Ok(valid_combinations
                .iter()
                .map(|combo| calculate_quantum_entanglement(&weights, combo))
                .min()
                .unwrap());
        }
    }

    Err(anyhow::anyhow!("No solution found"))
}

/// Checks if the remaining packages can be partitioned into `k` groups,
fn can_partition_remaining(
    all_weights: &[u32],
    used_indices: &[usize],
    target: u32,
    k: u32,
) -> bool {
    if k == 1 {
        return true; // If one group is left, its sum is guaranteed by the total.
    }

    let used_set: HashSet<usize> = used_indices.iter().cloned().collect();
    let remaining_weights: Vec<u32> = all_weights
        .iter()
        .enumerate()
        .filter_map(|(i, &w)| if !used_set.contains(&i) { Some(w) } else { None })
        .collect();

    // For k > 2 (i.e., 3 groups remaining), use a backtracking k-partition solver.
    let mut groups = vec![0; k as usize];
    let mut sorted_remaining = remaining_weights;
    sorted_remaining.sort_unstable_by(|a, b| b.cmp(a));

    can_k_partition_recursive(&sorted_remaining, 0, &mut groups, target)
}

/// Backtracking recursive solver for the k-partition problem.
fn can_k_partition_recursive(
    weights: &[u32],
    index: usize,
    groups: &mut [u32],
    target: u32,
) -> bool {
    if index == weights.len() {
        return true;
    }

    let current_weight = weights[index];
    for i in 0..groups.len() {
        if groups[i] + current_weight <= target {
            groups[i] += current_weight;
            if can_k_partition_recursive(weights, index + 1, groups, target) {
                return true;
            }
            groups[i] -= current_weight;
        }
        if groups[i] == 0 {
            break;
        }
    }

    false
}

/// Generates all combinations of a given `size` from `weights` that sum to `target`.
fn generate_combinations_with_sum(
    weights: &[u32],
    size: usize,
    target: u32,
    start_idx: usize,
    current_sum: u32,
    current_combo: &mut Vec<usize>,
    results: &mut Vec<Vec<usize>>,
) {
    if current_sum > target {
        return;
    }

    if current_combo.len() == size {
        if current_sum == target {
            results.push(current_combo.clone());
        }
        return;
    }

    if current_combo.len() + (weights.len() - start_idx) < size {
        return;
    }

    for i in start_idx..weights.len() {
        current_combo.push(i);
        generate_combinations_with_sum(
            weights,
            size,
            target,
            i + 1,
            current_sum + weights[i],
            current_combo,
            results,
        );
        current_combo.pop(); // Backtrack
    }
}

/// Calculates the quantum entanglement for a group of packages.
fn calculate_quantum_entanglement(weights: &[u32], indices: &[usize]) -> u64 {
    indices.iter().map(|&i| weights[i] as u64).product()
}
