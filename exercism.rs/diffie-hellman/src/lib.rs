use rand::seq::IteratorRandom;
use rand::thread_rng;

pub fn private_key(p: u64) -> u64 {
    (2..p).choose(&mut thread_rng()).unwrap()
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    exp_mod(g as u128, a, p as u128) as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    exp_mod(b_pub as u128, a, p as u128) as u64
}

fn exp_mod(base: u128, exp: u64, divisor: u128) -> u128 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        (exp_mod(base, exp / 2, divisor).pow(2)) % divisor
    } else {
        (base * exp_mod(base, exp - 1, divisor)) % divisor
    }
}
