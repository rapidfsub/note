use std::cmp::max;
use std::error::Error;
use std::io::stdin;

fn main() {
    let result = solve().unwrap_or(-1);
    println!("{result}");
}

fn solve() -> Result<i64, Box<dyn Error>> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let size: i64 = buffer.trim().parse()?;
    buffer.clear();

    let mut balance: i64 = 0;
    let mut result = None;
    let mut lower_bound: i64 = 0;
    for _ in 0..size {
        stdin().read_line(&mut buffer)?;
        let tokens: Vec<_> = buffer.split_whitespace().collect();
        let diff: i64 = tokens[0].parse()?;
        let value: i64 = tokens[1].parse()?;

        balance += diff;
        if diff < 0 {
            if balance < 0 {
                let withdraw = value - balance;
                lower_bound = max(lower_bound, value);
                match result {
                    Some(prev) => {
                        let next = gcd(prev, withdraw);
                        if next <= lower_bound {
                            Err("invalid")?;
                        }
                        result = Some(next);
                    }
                    None => {
                        result = Some(withdraw);
                    }
                }
                balance = value;
            } else if balance != value {
                Err("invalid")?;
            }
        } else if balance != value {
            Err("invalid")?;
        }

        buffer.clear();
    }
    Ok(result.unwrap_or(1))
}

fn gcd(x: i64, y: i64) -> i64 {
    if y > 0 {
        gcd(y, x % y)
    } else {
        x
    }
}
