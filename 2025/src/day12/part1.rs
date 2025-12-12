use anyhow::Result;
use aoc_lib::read_lines_keep_empty;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    r: i32,
    c: i32,
}

#[derive(Debug, Clone)]
struct Variation {
    points: Vec<Point>,
    height: usize,
    width: usize,
}

#[derive(Debug, Clone)]
struct Shape {
    id: usize,
    variations: Vec<Variation>,
    size: usize,
}

struct Query {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

pub fn solve() -> Result<usize> {
    let lines = read_lines_keep_empty("input/day12.in")?;
    let (shapes, queries) = parse_input(&lines);

    let mut success_count = 0;
    for query in queries {
        if can_fit(&shapes, &query) {
            success_count += 1;
        }
    }

    Ok(success_count)
}

fn parse_input(lines: &[String]) -> (Vec<Shape>, Vec<Query>) {
    let mut shapes = Vec::new();
    let mut queries = Vec::new();

    let mut current_shape_id = None;
    let mut current_shape_lines = Vec::new();

    for line in lines {
        let trim = line.trim();
        if trim.is_empty() {
            if let Some(id) = current_shape_id {
                shapes.push(create_shape(id, &current_shape_lines));
                current_shape_id = None;
                current_shape_lines.clear();
            }
            continue;
        }

        if trim.contains("x") && trim.contains(":") {
            if let Some(id) = current_shape_id {
                shapes.push(create_shape(id, &current_shape_lines));
                current_shape_id = None;
                current_shape_lines.clear();
            }

            let parts: Vec<&str> = trim.split(':').collect();
            let dims: Vec<usize> = parts[0]
                .split('x')
                .map(|s| s.parse().unwrap())
                .collect();
            let counts: Vec<usize> = parts[1]
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            queries.push(Query {
                width: dims[0],
                height: dims[1],
                counts,
            });
        } else if trim.ends_with(':') {
            if let Some(id) = current_shape_id {
                shapes.push(create_shape(id, &current_shape_lines));
                current_shape_lines.clear();
            }
            let id_str = trim.trim_end_matches(':');
            current_shape_id = Some(id_str.parse().unwrap());
        } else {
            current_shape_lines.push(trim.to_string());
        }
    }

    if let Some(id) = current_shape_id {
        shapes.push(create_shape(id, &current_shape_lines));
    }

    shapes.sort_by_key(|s| s.id);
    (shapes, queries)
}

fn create_shape(id: usize, lines: &[String]) -> Shape {
    let mut points = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                points.push(Point {
                    r: r as i32,
                    c: c as i32,
                });
            }
        }
    }

    let size = points.len();
    let mut variations = Vec::new();
    let mut seen = HashSet::new();

    let mut current = points;
    for _ in 0..2 {
        for _ in 0..4 {
            let normalized = normalize(&current);
            let key = normalized_key(&normalized);
            if seen.insert(key) {
                let max_r = normalized.iter().map(|p| p.r).max().unwrap_or(0);
                let max_c = normalized.iter().map(|p| p.c).max().unwrap_or(0);
                variations.push(Variation {
                    points: normalized,
                    height: (max_r + 1) as usize,
                    width: (max_c + 1) as usize,
                });
            }
            current = rotate(&current);
        }
        current = flip(&current);
    }

    Shape {
        id,
        variations,
        size,
    }
}

fn normalize(points: &[Point]) -> Vec<Point> {
    if points.is_empty() {
        return vec![];
    }
    let min_r = points.iter().map(|p| p.r).min().unwrap();
    let min_c = points.iter().map(|p| p.c).min().unwrap();

    let mut norm: Vec<Point> = points
        .iter()
        .map(|p| Point {
            r: p.r - min_r,
            c: p.c - min_c,
        })
        .collect();
    norm.sort();
    norm
}

fn normalized_key(points: &[Point]) -> Vec<(i32, i32)> {
    points.iter().map(|p| (p.r, p.c)).collect()
}

fn rotate(points: &[Point]) -> Vec<Point> {
    points.iter().map(|p| Point { r: p.c, c: -p.r }).collect()
}

fn flip(points: &[Point]) -> Vec<Point> {
    points.iter().map(|p| Point { r: p.r, c: -p.c }).collect()
}

fn can_fit(shapes: &[Shape], query: &Query) -> bool {
    let mut to_place = Vec::new();
    let mut total_area = 0;

    for (id, &count) in query.counts.iter().enumerate() {
        if count > 0 {
            let shape = &shapes[id];
            for _ in 0..count {
                to_place.push(shape);
                total_area += shape.size;
            }
        }
    }

    if total_area > query.width * query.height {
        return false;
    }

    to_place.sort_by(|a, b| b.size.cmp(&a.size));

    let mut grid = vec![false; query.width * query.height];
    backtrack(0, &to_place, &mut grid, query.width, query.height)
}

fn backtrack(
    idx: usize,
    pieces: &[&Shape],
    grid: &mut [bool],
    width: usize,
    height: usize,
) -> bool {
    if idx == pieces.len() {
        return true;
    }

    let piece = pieces[idx];
    
    let start_pos = if idx > 0 && pieces[idx].id == pieces[idx - 1].id {
        0 
    } else {
        0
    };

    let len = grid.len();

    for variant in &piece.variations {
        if variant.width > width || variant.height > height {
            continue;
        }

        for pos in start_pos..len {
            let r = pos / width;
            let c = pos % width;

            if r + variant.height > height || c + variant.width > width {
                continue;
            }

            if can_place_variant(grid, width, r, c, variant) {
                place_variant(grid, width, r, c, variant, true);
                
                if backtrack(idx + 1, pieces, grid, width, height) {
                    return true;
                }
                
                place_variant(grid, width, r, c, variant, false);
            }
        }
    }

    false
}

fn can_place_variant(
    grid: &[bool],
    width: usize,
    r: usize,
    c: usize,
    variant: &Variation,
) -> bool {
    for p in &variant.points {
        let gr = r + p.r as usize;
        let gc = c + p.c as usize;
        if grid[gr * width + gc] {
            return false;
        }
    }
    true
}

fn place_variant(
    grid: &mut [bool],
    width: usize,
    r: usize,
    c: usize,
    variant: &Variation,
    val: bool,
) {
    for p in &variant.points {
        let gr = r + p.r as usize;
        let gc = c + p.c as usize;
        grid[gr * width + gc] = val;
    }
}
