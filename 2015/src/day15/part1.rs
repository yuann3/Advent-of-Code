use anyhow::Result;
use aoc_lib::read_lines;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
}

impl Ingredient {
    fn parse_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split(&[' ', ',', ':']).collect();

        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        for i in 0..parts.len() {
            match parts[i] {
                "capacity" => capacity = parts[i + 1].parse()?,
                "durability" => durability = parts[i + 1].parse()?,
                "flavor" => flavor = parts[i + 1].parse()?,
                "texture" => texture = parts[i + 1].parse()?,
                _ => continue,
            }
        }

        Ok(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
        })
    }
}

fn calculate_score(ingredients: &[Ingredient], amounts: &[i32; 4]) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for (ingredient, &amount) in ingredients.iter().zip(amounts.iter()) {
        capacity += ingredient.capacity * amount;
        durability += ingredient.durability * amount;
        flavor += ingredient.flavor * amount;
        texture += ingredient.texture * amount;
    }

    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);

    capacity * durability * flavor * texture
}

pub fn solve() -> Result<i32> {
    let lines = read_lines("input/day15.in")?;

    let ingredients: Result<Vec<Ingredient>, _> = lines
        .iter()
        .map(|line| Ingredient::parse_line(line))
        .collect();

    let ingredients = ingredients?;

    let mut max_score = 0;
    for a in 0..=100 {
        for b in 0..=(100 - a) {
            for c in 0..=(100 - a - b) {
                let d = 100 -a -b -c;

                let amounts = [a, b, c, d];
                let score = calculate_score(&ingredients, &amounts);
                max_score = max_score.max(score)
            }
        }
    }

    Ok(max_score)
}
