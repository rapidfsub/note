pub fn is_armstrong_number(num: u32) -> bool {
    let exp = (num as f64).log10() as u32 + 1;
    let mut result: u32 = 0;
    let mut dividend = num;
    while dividend > 0 {
        let digit = dividend % 10;
        match digit
            .checked_pow(exp)
            .and_then(|power| result.checked_add(power))
        {
            Some(value) => result = value,
            None => return false,
        }
        dividend /= 10;
    }
    result == num
}
