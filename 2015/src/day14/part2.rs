use anyhow::Result;
use aoc_lib::read_lines;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
}

impl Reindeer {
    fn parse_from_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let name = parts[0].to_string();
        let speed = parts[3].parse::<i32>()?;
        let fly_time = parts[6].parse::<i32>()?;
        let rest_time = parts[13].parse::<i32>()?;

        Ok(Reindeer {
            name,
            speed,
            fly_time,
            rest_time,
        })
    }

    fn final_distance(&self, total_time: i32) -> i32 {
        let cycle_time = self.fly_time + self.rest_time;
        let complete_cycles = total_time / cycle_time;
        let remaining_time = total_time % cycle_time;
        let distance_from_cycles = complete_cycles * self.speed * self.fly_time;

        let flying_time_in_partial = std::cmp::min(remaining_time, self.fly_time);
        let distance_from_partial = self.speed * flying_time_in_partial;

        distance_from_cycles + distance_from_partial
    }
}

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day14.in")?;
    let race_time = 2503;

    let mut reindeer: Vec<Reindeer> = Vec::new();
    for line in lines {
        reindeer.push(Reindeer::parse_from_line(&line)?);
    }

    let mut points: Vec<i32> = vec![0; reindeer.len()];

    for second in 1..race_time {
        let mut distance: Vec<i32> = Vec::new();
        for deer in &reindeer {
            distance.push(deer.final_distance(second));
        }

        let max_distance = *distance.iter().max().unwrap();

        for i in 0..reindeer.len() {
            if distance[i] == max_distance {
                points[i] += 1;
            }
        }

        if second <= 5 || second == 140 || second == 1000 {
            println!(
                "Second {}: distance = {:?}, points = {:?}",
                second, distance, points
            );
        }
    }

    for i in 0..reindeer.len() {
        println!("{}: {} points", reindeer[i].name, points[i]);
    }

    Ok(*points.iter().max().unwrap())
}
