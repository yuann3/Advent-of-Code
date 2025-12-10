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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

#[derive(Clone, Copy)]
struct Rat { n: i64, d: i64 }

impl Rat {
    fn new(n: i64, d: i64) -> Self {
        if d == 0 { return Rat { n: 0, d: 1 }; }
        let g = gcd(n, d);
        let (n, d) = (n / g, d / g);
        if d < 0 { Rat { n: -n, d: -d } } else { Rat { n, d } }
    }
    fn zero() -> Self { Rat { n: 0, d: 1 } }
    fn is_zero(&self) -> bool { self.n == 0 }
    fn sub(self, other: Rat) -> Rat { Rat::new(self.n * other.d - other.n * self.d, self.d * other.d) }
    fn mul(self, other: Rat) -> Rat { Rat::new(self.n * other.n, self.d * other.d) }
    fn div(self, other: Rat) -> Rat { Rat::new(self.n * other.d, self.d * other.n) }
    fn to_i64(self) -> Option<i64> { if self.d == 1 { Some(self.n) } else { None } }
}

fn min_presses(line: &str) -> i64 {
    let (buttons, targets) = parse_line(line);
    let m = targets.len();
    let n = buttons.len();
    
    let mut mat: Vec<Vec<Rat>> = vec![vec![Rat::zero(); n + 1]; m];
    for (j, btn) in buttons.iter().enumerate() {
        for &i in btn {
            mat[i][j] = Rat::new(1, 1);
        }
    }
    for i in 0..m {
        mat[i][n] = Rat::new(targets[i], 1);
    }

    let mut pivot_cols = vec![];
    let mut row = 0;
    for col in 0..n {
        if row >= m { break; }
        let pivot_row = (row..m).find(|&r| !mat[r][col].is_zero());
        if pivot_row.is_none() { continue; }
        mat.swap(row, pivot_row.unwrap());
        let scale = mat[row][col];
        for c in 0..=n { mat[row][c] = mat[row][c].div(scale); }
        for r in 0..m {
            if r != row && !mat[r][col].is_zero() {
                let factor = mat[r][col];
                for c in 0..=n { mat[r][c] = mat[r][c].sub(factor.mul(mat[row][c])); }
            }
        }
        pivot_cols.push(col);
        row += 1;
    }

    let free_cols: Vec<usize> = (0..n).filter(|c| !pivot_cols.contains(c)).collect();
    let rank = pivot_cols.len();
    let max_free: i64 = targets.iter().max().copied().unwrap_or(0);
    let mut best = i64::MAX;

    fn search(
        free_cols: &[usize], idx: usize, vals: &mut Vec<i64>,
        mat: &[Vec<Rat>], pivots: &[usize], n: usize, rank: usize,
        max_v: i64, best: &mut i64,
    ) {
        if idx == free_cols.len() {
            let mut total: i64 = vals.iter().sum();
            if total >= *best { return; }
            for r in 0..rank {
                let mut val = mat[r][n];
                for (i, &fc) in free_cols.iter().enumerate() {
                    val = val.sub(mat[r][fc].mul(Rat::new(vals[i], 1)));
                }
                match val.to_i64() {
                    Some(v) if v >= 0 => total += v,
                    _ => return,
                }
                if total >= *best { return; }
            }
            *best = total;
            return;
        }
        for v in 0..=max_v {
            vals.push(v);
            search(free_cols, idx + 1, vals, mat, pivots, n, rank, max_v, best);
            vals.pop();
        }
    }

    search(&free_cols, 0, &mut vec![], &mat, &pivot_cols, n, rank, max_free, &mut best);
    best
}

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day10.in")?;
    Ok(lines.iter().map(|l| min_presses(l)).sum::<i64>() as u64)
}
