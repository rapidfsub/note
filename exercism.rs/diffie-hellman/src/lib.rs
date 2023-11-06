use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    exp_mod(g as u128, a, p as u128) as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    exp_mod(b_pub as u128, a, p as u128) as u64
}

fn exp_mod(mut base: u128, mut exp: u64, divisor: u128) -> u128 {
    let mut result = 1;
    while exp > 0 {
        if exp & 1 > 0 {
            result = result * base % divisor;
        }
        base = base.pow(2) % divisor;
        exp = exp >> 1;
    }
    result
}
