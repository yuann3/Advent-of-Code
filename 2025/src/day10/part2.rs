use anyhow::Result;
use aoc_lib::read_lines;

fn parse_line(line: &str) -> (Vec<Vec<usize>>, Vec<i64>) {
    let buttons: Vec<Vec<usize>> = line
        .split('(')
        .skip(1)
        .map(|s| {
            s.split(')').next().unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let targets: Vec<i64> = line
        .split('{').nth(1).unwrap()
        .split('}').next().unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    (buttons, targets)
}

fn search(
    buttons: &[Vec<usize>],
    targets: &[i64],
    current: &mut [i64],
    idx: usize,
    total: i64,
    best: &mut i64,
) {
    if current == targets {
        *best = (*best).min(total);
        return;
    }
    if idx >= buttons.len() || total >= *best {
        return;
    }
    if current.iter().zip(targets).any(|(c, t)| c > t) {
        return;
    }

    let max_press = buttons[idx]
        .iter()
        .map(|&i| targets[i] - current[i])
        .min()
        .unwrap_or(0);

    for presses in 0..=max_press {
        for &i in &buttons[idx] {
            current[i] += presses;
        }
        search(buttons, targets, current, idx + 1, total + presses, best);
        for &i in &buttons[idx] {
            current[i] -= presses;
        }
    }
}

fn min_presses(line: &str) -> i64 {
    let (buttons, targets) = parse_line(line);
    let mut current = vec![0i64; targets.len()];
    let mut best = i64::MAX;
    search(&buttons, &targets, &mut current, 0, 0, &mut best);
    best
}

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day10.in")?;
    let total: i64 = lines.iter().map(|l| min_presses(l)).sum();
    Ok(total as u64)
}
