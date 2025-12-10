use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u64> {
    let lines = read_lines("input/day9.in")?;
    let points: Vec<(i64, i64)> = lines
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut edges = Vec::new();
    for i in 0..points.len() {
        edges.push((points[i], points[(i + 1) % points.len()]));
    }

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let area = width * height;

            if area <= max_area {
                continue;
            }

            let mut valid = true;

            for &((ex1, ey1), (ex2, ey2)) in &edges {
                let e_min_x = ex1.min(ex2);
                let e_max_x = ex1.max(ex2);
                let e_min_y = ey1.min(ey2);
                let e_max_y = ey1.max(ey2);

                if ex1 == ex2 {
                    if ex1 > min_x && ex1 < max_x {
                        let overlap_min = min_y.max(e_min_y);
                        let overlap_max = max_y.min(e_max_y);
                        if overlap_min < overlap_max {
                            valid = false;
                            break;
                        }
                    }
                } else {
                    if ey1 > min_y && ey1 < max_y {
                        let overlap_min = min_x.max(e_min_x);
                        let overlap_max = max_x.min(e_max_x);
                        if overlap_min < overlap_max {
                            valid = false;
                            break;
                        }
                    }
                }
            }

            if !valid {
                continue;
            }

            let mid_x2 = min_x + max_x;
            let mid_y2 = min_y + max_y;

            let mut on_edge = false;
            for &((ex1, ey1), (ex2, ey2)) in &edges {
                let e2_min_x = 2 * ex1.min(ex2);
                let e2_max_x = 2 * ex1.max(ex2);
                let e2_min_y = 2 * ey1.min(ey2);
                let e2_max_y = 2 * ey1.max(ey2);

                if mid_x2 >= e2_min_x && mid_x2 <= e2_max_x && mid_y2 >= e2_min_y && mid_y2 <= e2_max_y {
                    on_edge = true;
                    break;
                }
            }

            if !on_edge {
                let mut crossings = 0;
                for &((ex1, ey1), (ex2, ey2)) in &edges {
                    if ex1 == ex2 {
                        let edge_x2 = 2 * ex1;
                        if edge_x2 > mid_x2 {
                            let e_y_min = ey1.min(ey2) * 2;
                            let e_y_max = ey1.max(ey2) * 2;
                            if mid_y2 >= e_y_min && mid_y2 < e_y_max {
                                crossings += 1;
                            }
                        }
                    }
                }
                if crossings % 2 == 0 {
                    valid = false;
                }
            }

            if valid {
                max_area = area;
            }
        }
    }

    Ok(max_area as u64)
}
