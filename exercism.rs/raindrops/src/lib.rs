pub fn raindrops(n: u32) -> String {
    let result: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|(divisor, _sound)| n % divisor == 0)
        .map(|(_divisor, sound)| *sound)
        .collect();
    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
