
pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut count = 1;
    let mut prime_nr = 3;
    while count < n {
        prime_nr += 2;
        if is_prime(prime_nr) {
            count += 1;
        }
    }
    prime_nr
}

fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n < 2 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt().floor() as u32;
    let mut i = 5;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
