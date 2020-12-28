use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    BigUint::from(g)
        .modpow(&BigUint::from(a), &BigUint::from(p))
        .to_u64()
        .unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    BigUint::from(b_pub)
        .modpow(&BigUint::from(a), &BigUint::from(p))
        .to_u64()
        .unwrap()
}
