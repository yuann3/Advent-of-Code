use md5::{Digest, Md5};
use std::io;

fn calculate_md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn solve() -> io::Result<i32> {
    let input = "ckczppom";
    let mut number = 1;

    loop {
        let result = format!("{}{}", input, number);
        let hash = calculate_md5(&result);

        if hash.starts_with("000000") {
            return Ok(number);
        }
        number += 1;
    }
}
