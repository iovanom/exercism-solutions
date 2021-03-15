use evalexpr::{eval, Value};
use itertools::Itertools;
use std::char;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let first_letters: Vec<char> = input
        .split(" ")
        .into_iter()
        .filter_map(|word| word.chars().nth(0))
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect();

    let letters: Vec<char> = input
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some(c.to_ascii_uppercase())
            } else {
                None
            }
        })
        .unique()
        .collect();

    // check if have more than 10 letters
    if letters.len() > 10 {
        return None;
    }
    for digits in (0..10).permutations(letters.len()) {
        let solution: HashMap<char, u8> = letters.clone().into_iter().zip(digits).collect();
        if first_letters
            .clone()
            .into_iter()
            .any(|c| solution.get(&c).unwrap().to_owned() == 0)
        {
            continue;
        }
        let expression: String = input
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    char::from_digit(
                        solution
                            .get(&c.to_ascii_uppercase())
                            .unwrap()
                            .to_owned()
                            .into(),
                        10,
                    )
                    .unwrap()
                } else {
                    c
                }
            })
            .collect();
        println!("{:?}", expression);
        if eval(&expression) == Ok(Value::from(true)) {
            return Some(solution);
        }
    }
    None
}
