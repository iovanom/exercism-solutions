pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 { return true }
    let factor = ((num as f32).log10().floor() + 1.0) as u32;
    (0..factor).map(|n| (num / 10u32.pow(n) % 10).pow(factor)).sum::<u32>() == num
}
