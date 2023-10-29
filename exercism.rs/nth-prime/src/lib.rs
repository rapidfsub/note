pub fn nth(n: u32) -> u32 {
    let mut result = 2;
    let mut i = 0;
    loop {
        if is_prime(result) {
            if n == i {
                return result;
            } else {
                i += 1;
            }
        }
        result += 1;
    }
}

fn is_prime(n: u32) -> bool {
    for i in 2..=sqrt_floor(n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn sqrt_floor(n: u32) -> u32 {
    (n as f64).powf(0.5).floor() as u32
}
