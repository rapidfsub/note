use std::error::Error;
use std::io::stdin;

const LIMIT: u64 = 1_000_000_007;

fn main() {
    let result = solve().unwrap();
    println!("{result}");
}

fn solve() -> Result<u64, Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let tokens: Vec<_> = buf.trim().split_whitespace().collect();
    let width: usize = tokens[0].parse()?;
    let height: usize = tokens[1].parse()?;
    buf.clear();

    let mut board: Vec<Vec<char>> = vec![];
    for i in 0..width {
        stdin().read_line(&mut buf)?;

        board.push(vec![]);
        let squares: Vec<_> = buf.trim().chars().collect();
        for j in 0..height {
            board[i].push(squares[j]);
        }

        buf.clear();
    }

    let mut count = 1;
    for (i, line) in board.iter().enumerate() {
        for (j, color) in line.iter().enumerate() {
            if i > 0 && *color != board[i - 1][j] {
                continue;
            } else if j > 0 && *color != board[i][j - 1] {
                continue;
            } else if i + 1 < board.len() && *color != board[i + 1][j] {
                continue;
            } else if j + 1 < board[i].len() && *color != board[i][j + 1] {
                continue;
            } else {
                count = count * 2 % LIMIT;
            }
        }
    }
    Ok(count)
}
