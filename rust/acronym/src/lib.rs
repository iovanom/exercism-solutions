pub fn abbreviate(phrase: &str) -> String {
    phrase.split(&[' ', '-'][..])
        .map(|i| {
            let all_upper = i == i.to_uppercase();
            i.chars()
             .filter(|c| c.is_alphabetic())
             .enumerate()
             .filter(|&(i, c)| i == 0 || !all_upper && c.is_uppercase())
             .map(|(_, c)| c)
             .collect::<String>()
        }).collect::<String>().to_uppercase()
}
