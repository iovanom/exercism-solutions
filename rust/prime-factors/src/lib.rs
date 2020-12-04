pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut test_factor = 2;
    while n > 1 {
       if n % test_factor == 0 {
           n = n / test_factor;
           factors.push(test_factor);
       } else {
           test_factor += 1;
       }
    }
    factors
}
