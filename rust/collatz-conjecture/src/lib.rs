pub fn collatz(mut n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }

    let mut steps = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        steps += 1;
    }
    Some(steps)
}

pub fn collatz_recurs(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz_recurs(n / 2).map(|x| x + 1),
        n => collatz_recurs(3 * n + 1).map(|x| x + 1),
    }
}
