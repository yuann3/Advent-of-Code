use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<String> {
    const KEYPAD: [[char; 5]; 5] = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];
    let lines = read_lines("input/day2.in")?;
    let mut x: i32 = 0;
    let mut y: i32 = 2;
    let mut code = String::new();

    for line in lines {
        for instruction in line.chars() {
            let (mut nx, mut ny) = (x, y);
            match instruction {
                'U' => ny -= 1,
                'D' => ny += 1,
                'L' => nx -= 1,
                'R' => nx += 1,
                _ => (),
            }

            if (0..=4).contains(&nx)
                && (0..=4).contains(&ny)
                && KEYPAD[ny as usize][nx as usize] != ' '
            {
                x = nx;
                y = ny;
            }
        }

        code.push(KEYPAD[y as usize][x as usize]);
    }

    Ok(code)
}
