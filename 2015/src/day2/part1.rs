use aoc_lib::read_lines;
use anyhow::Result;

pub struct Prism {
    pub length: i32,
    pub width: i32,
    pub height: i32,
}

impl Prism {
    pub fn new(length: i32, width: i32, height: i32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    pub fn area(&self) -> i32 {
        let sides = [
            self.length * self.width,
            self.height * self.width,
            self.height * self.length,
        ];

        let smallest = sides.iter().min().unwrap();
        let surface_area = 2 * sides.iter().sum::<i32>();

        smallest + surface_area
    }
}

pub fn parse_line(line: &str) -> Option<Prism> {
    let parts: Vec<&str> = line.split('x').collect();
    if parts.len() != 3 {
        return None;
    }

    let [length, width, height] = parts.as_slice() else {
        return None;
    };
    Some(Prism::new(
        length.parse().ok()?,
        width.parse().ok()?,
        height.parse().ok()?,
    ))
}

pub fn solve() -> Result<i32> {
    let input = read_lines("input/day2p1.txt")?;

    let total_area = input
        .iter()
        .enumerate()
        .try_fold(0, |acc, (line_number, line)| {
            parse_line(line)
                .map(|prism| acc + prism.area())
                .ok_or_else(|| {
                    println!("Error in line {}: {}", line_number + 1, line);
                    anyhow::anyhow!("Invalid input")
                })
        })?;

    Ok(total_area)
}
