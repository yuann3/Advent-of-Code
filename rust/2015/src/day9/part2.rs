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
        distances.insert((destination.to_string(), source.to_string()), distance);
    }

    let cities_vec: Vec<String> = cities.into_iter().collect();
    let n = cities_vec.len();

    let mut indices: Vec<usize> = (0..n).collect();
    let mut c: Vec<usize> = vec![0; n];
    // part 2 longest
    let mut longest_distence = 0;
    let mut i = 0;

    while i < n {
        if c[i] < i {
            let to_swap = if i % 2 == 0 { 0 } else { c[i] };
            indices.swap(to_swap, i);

            let mut total_distance = 0;

            for j in 0..n - 1 {
                let current_city = &cities_vec[indices[j]];
                let next_city = &cities_vec[indices[j + 1]];

                if let Some(&dist) = distances.get(&(current_city.clone(), next_city.clone())) {
                    total_distance += dist;
                }
            }

            longest_distence = longest_distence.max(total_distance);

            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    Ok(longest_distence)
}
