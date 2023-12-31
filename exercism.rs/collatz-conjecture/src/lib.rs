pub fn collatz(mut n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    let mut count = 0;
    loop {
        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n /= 2;
            count += 1;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
            count += 1;
        }
    }
    Some(count)
}
