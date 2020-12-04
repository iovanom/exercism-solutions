pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    list.windows(2)
        .map(|r| format!("For want of a {} the {} was lost.\n", r[0], r[1]))
        .collect::<String>()
        + &format!("And all for the want of a {}.", list[0])
}
