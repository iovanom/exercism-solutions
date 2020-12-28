use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: HashSet<_> = (2..=upper_bound).collect();
    let max_limit = (upper_bound as f64).sqrt().floor() as u64;
    for i in 2..=max_limit {
        if numbers.contains(&i) {
            let mut j = i.pow(2);
            let mut iter = 1;
            while j <= upper_bound {
                numbers.remove(&j);
                j = i.pow(2) + i * iter;
                iter += 1;
            }
        }
    }
    let mut numbers: Vec<_> = numbers.iter().copied().collect();
    numbers.sort();
    numbers
}
