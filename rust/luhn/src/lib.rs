/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // validate code
    if code.chars().filter(|&c| c.is_digit(10)).count() < 2
        || code.chars().any(|c| !c.is_digit(10) && c != ' ')
    {
        return false;
    }
    let sum = code
        .chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 0 {
                d
            } else if d + d > 9 {
                d + d - 9
            } else {
                d + d
            }
        })
        .sum::<u32>();
    sum % 10 == 0
}
