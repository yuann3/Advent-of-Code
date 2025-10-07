use anyhow::Result;
use md5::compute;

pub fn solve() -> Result<String> {
    let door_id = "ugkcyxxp";
    let mut password = String::with_capacity(8);
    let mut index: u64 = 0;

    while password.len() < 8 {
        let input = format!("{}{}", door_id, index);
        let digest = compute(input.as_bytes());
        let hash = format!("{:x}", digest);

        if hash.starts_with("00000") {
            let ch = hash.as_bytes()[5] as char;
            password.push(ch);
        }
        index += 1;
    }

    Ok(password)
}
