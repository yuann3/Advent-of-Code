use anyhow::Result;
use md5::compute;

pub fn solve() -> Result<String> {
    let door_id = "ugkcyxxp";
    let mut password: Vec<Option<char>> = vec![None; 8];
    let mut index: u64 = 0;

    while password.iter().any(|c| c.is_none()) {
        let input = format!("{}{}", door_id, index);
        let digest = compute(input.as_bytes());
        let hash = format!("{:x}", digest);

        if hash.starts_with("00000") {
            let pos_char = hash.as_bytes()[5] as char;
            if let Some(pos) = pos_char.to_digit(10) {
                let pos = pos as usize;
                if pos < 8 && password[pos].is_none() {
                    let ch = hash.as_bytes()[6] as char;
                    password[pos] = Some(ch);
                }
            }
        }

        index += 1;
    }

    let password_str: String = password.into_iter().map(|c| c.unwrap()).collect();
    Ok(password_str)
}
