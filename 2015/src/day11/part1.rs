use anyhow::Result;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn solve() -> Result<String> {
    let input = read_to_string("input/day11.in")?;
    let mut password: Vec<char> = input.trim().chars().collect();

    loop {
        increment_password(&mut password);
        if is_valid_password(&password) {
            break;
        }
    }

    let result: String = password.iter().collect();
    println!("Next valid password: {}", result);
    Ok(result)
}

fn increment_password(password: &mut Vec<char>) {
    let mut pos = password.len() - 1;

    loop {
        if password[pos] != 'z' {
            password[pos] = char::from(password[pos] as u8 + 1);
            break;
        } else {
            password[pos] = 'a';
            if pos == 0 {
                break;
            }
            pos -= 1;
        }
    }
}

fn is_valid_password(password: &[char]) -> bool {
    has_increasing_straight(password)
        && !has_forbidden_letters(password)
        && has_two_pairs(password)
}

fn has_increasing_straight(password: &[char]) -> bool {
    password.windows(3).any(|window| {
        window[0] as u8 + 1 == window[1] as u8
            && window[1] as u8 + 1 == window[2] as u8
    })
}

fn has_forbidden_letters(password: &[char]) -> bool {
    password.iter().any(|&c| c == 'i' || c == 'o' || c == 'l')
}

fn has_two_pairs(password: &[char]) -> bool {
    let mut pairs = HashSet::new();
    let mut i = 0;

    while i < password.len() - 1 {
        if password[i] == password[i + 1] {
            pairs.insert(password[i]);
            i += 2;
        } else {
            i += 1;
        }
    }

    pairs.len() >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_password() {
        let mut pw = "xx".chars().collect();
        increment_password(&mut pw);
        assert_eq!(pw.iter().collect::<String>(), "xy");

        increment_password(&mut pw);
        assert_eq!(pw.iter().collect::<String>(), "xz");

        increment_password(&mut pw);
        assert_eq!(pw.iter().collect::<String>(), "ya");
    }

    #[test]
    fn test_has_increasing_straight() {
        assert!(has_increasing_straight(&"hijklmmn".chars().collect::<Vec<_>>()));
        assert!(has_increasing_straight(&"abcdef".chars().collect::<Vec<_>>()));
        assert!(!has_increasing_straight(&"abbceffg".chars().collect::<Vec<_>>()));
        assert!(!has_increasing_straight(&"abbcegjk".chars().collect::<Vec<_>>()));
    }

    #[test]
    fn test_has_forbidden_letters() {
        assert!(has_forbidden_letters(&"hijklmmn".chars().collect::<Vec<_>>()));
        assert!(!has_forbidden_letters(&"abbceffg".chars().collect::<Vec<_>>()));
        assert!(has_forbidden_letters(&"hello".chars().collect::<Vec<_>>()));
    }

    #[test]
    fn test_has_two_pairs() {
        assert!(has_two_pairs(&"abbceffg".chars().collect::<Vec<_>>()));
        assert!(!has_two_pairs(&"abbcegjk".chars().collect::<Vec<_>>()));
        assert!(has_two_pairs(&"aabbcc".chars().collect::<Vec<_>>()));
        assert!(has_two_pairs(&"aaabbb".chars().collect::<Vec<_>>()));
    }

    #[test]
    fn test_examples() {
        let pw1: Vec<char> = "hijklmmn".chars().collect();
        assert!(has_increasing_straight(&pw1));
        assert!(has_forbidden_letters(&pw1));
        assert!(!is_valid_password(&pw1));

        let pw2: Vec<char> = "abbceffg".chars().collect();
        assert!(!has_increasing_straight(&pw2));
        assert!(!has_forbidden_letters(&pw2));
        assert!(has_two_pairs(&pw2));
        assert!(!is_valid_password(&pw2));

        let pw3: Vec<char> = "abbcegjk".chars().collect();
        assert!(!has_two_pairs(&pw3));
        assert!(!is_valid_password(&pw3));
    }
}
