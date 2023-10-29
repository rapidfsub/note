pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut dividend = n;
    let mut divisor = 2;
    while dividend > 1 {
        if dividend % divisor == 0 {
            result.push(divisor);
            dividend /= divisor;
        } else {
            divisor += 1;
        }
    }
    result
}
