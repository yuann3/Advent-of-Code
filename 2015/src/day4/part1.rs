use md5::{Digest, Md5};
use anyhow::Result;

fn calculate_md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn solve() -> Result<i32> {
    let input = "ckczppom";
    let mut number = 1;

    loop {
        let result = format!("{}{}", input, number);
        let hash = calculate_md5(&result);

        if hash.starts_with("00000") {
            return Ok(number);
        }
        number += 1;
    }
}
