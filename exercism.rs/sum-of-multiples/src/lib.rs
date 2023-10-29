pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result = 0;
    for dividend in 1..limit {
        for &divisor in factors {
            if divisor > 0 && dividend % divisor == 0 {
                result += dividend;
                break;
            }
        }
    }
    result
}
