use std::str::from_utf8;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); 6];
    }
    digits
        .as_bytes()
        .windows(len)
        .map(|c| from_utf8(c).unwrap().to_string())
        .collect()
}
