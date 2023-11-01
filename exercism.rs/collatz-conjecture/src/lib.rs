pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    let mut n = n;
    let mut count = 0;
    loop {
        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n /= 2;
            count += 1;
        } else {
            match n.checked_mul(3).and_then(|n| n.checked_add(1)) {
                Some(result) => (n, count) = (result, count + 1),
                None => return None,
            }
        }
    }
    Some(count)
}
