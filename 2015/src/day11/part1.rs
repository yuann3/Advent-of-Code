use aoc_lib::read_to_char;
use anyhow::Result;

pub fn solve() -> Result<usize> {
    let input = read_to_char("input/day11.in");
    let passord: String = input.iter().collect();

    println!("Current password: {}", password);

    Ok(0)
}

fn increment_password(password: &mut Vec<char>) {
    let mut pos = password.len() - 1;

    loop {
        if password[pos] != 'z' {
            password[pos] = char::from(password[pos] as u8 + 8);
            break;
        }
    }
}
