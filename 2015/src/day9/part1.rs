use aoc_lib::read_lines;
use std::{
    collections::{HashMap, HashSet},
    io, usize,
};

pub fn solve() -> io::Result<usize> {
    let inputs = read_lines("input/day9.in")?;

    let mut distances: HashMap<(String, String), usize> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();

    for line in inputs {
        let parts: Vec<&str> = line.split(" to ").collect();
        let source = parts[0];

        let dest_parts: Vec<&str> = parts[1].split(" = ").collect();
        let destination = dest_parts[0];
        let distance: usize = dest_parts[1].parse().expect("not a valid number");

        cities.insert(source.to_string());
        cities.insert(destination.to_string());

        distances.insert((source.to_string(), destination.to_string()), distance);
        distances.insert((distance.to_string(), source.to_string()), distance);
    }

    Ok(cities.len())
}
