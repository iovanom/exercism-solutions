use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort();

    possible_anagrams.iter()
        .cloned()
        .filter(|posible| {
            let mut posible_chars: Vec<char> = posible.to_lowercase().chars().collect();
            posible_chars.sort();
            posible_chars == word_chars && word.to_lowercase() != posible.to_lowercase()
        }).collect()
}
