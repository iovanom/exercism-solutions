pub fn verse(n: u32) -> String {
    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        ),
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n"
        ),
        _ => format!(
            "{} bottles of beer on the wall, {0} bottles of beer.\n\
            Take one down and pass it around, {} bottle{} of beer on the wall.\n",
            n,
            n - 1,
            if n - 1 == 1 { "" } else { "s" },
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|i| if i == end { verse(i) } else { verse(i) + "\n" })
        .collect()
}
