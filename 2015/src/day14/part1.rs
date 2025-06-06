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

    let mut max_distance = 0;

    for line in lines {
        let reindeer = Reindeer::parse_from_line(&line)?;
        let distance = reindeer.final_distance(race_time);

        println!("{}: {} km", reindeer.name, distance);

        max_distance = std::cmp::max(max_distance, distance);
    }

    Ok(max_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let comet = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };

        let dancer = Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        // Test after 1000 seconds
        assert_eq!(comet.final_distance(1000), 1120);
        assert_eq!(dancer.final_distance(1000), 1056);

        // Test intermediate values
        assert_eq!(comet.final_distance(1), 14);
        assert_eq!(dancer.final_distance(1), 16);
        assert_eq!(comet.final_distance(10), 140);
        assert_eq!(dancer.final_distance(10), 160);
    }
}
