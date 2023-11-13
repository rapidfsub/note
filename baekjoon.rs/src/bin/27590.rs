use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<_> = input.split_whitespace().collect();

    let sun_ago: i32 = tokens[0].parse().unwrap();
    let sun_period: i32 = tokens[1].parse().unwrap();
    let moon_ago: i32 = tokens[2].parse().unwrap();
    let moon_period: i32 = tokens[3].parse().unwrap();

    let mut result = 0;
    while (result + sun_ago) % sun_period != 0 || (result + moon_ago) % moon_period != 0 {
        result += 1;
    }
    println!("{result}");
}
