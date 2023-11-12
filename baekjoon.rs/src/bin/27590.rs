use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<_> = input.split_whitespace().collect();
    let sun_ago = -tokens[0].parse::<i32>().unwrap();
    let sun_period: i32 = tokens[1].parse().unwrap();
    let mut sun_past = 0;
    let moon_ago = -tokens[2].parse::<i32>().unwrap();
    let moon_period: i32 = tokens[3].parse().unwrap();
    let mut moon_past = 0;
    if sun_period < moon_period {
        sun_past += sun_period;
    } else {
        moon_past += moon_period;
    }
    loop {
        let sun = sun_ago + sun_past;
        let moon = moon_ago + moon_past;
        if sun > moon {
            moon_past += moon_period;
        } else if sun < moon {
            sun_past += sun_period;
        } else {
            break;
        }
    }
    let result = sun_ago + sun_past;
    println!("{result}");
}
