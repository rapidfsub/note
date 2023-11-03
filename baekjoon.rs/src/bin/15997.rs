use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.trim().split('\n').collect();
    let teams: Vec<_> = lines[0].split_ascii_whitespace().collect();
    let count = teams.len();
    let mut rates = vec![vec![[0f64; 3]; count]; count];
    for line in lines.iter().skip(1) {
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let t1 = tokens[0];
        let t2 = tokens[1];
        let i_t1 = teams.iter().position(|t| *t == t1).unwrap();
        let i_t2 = teams.iter().position(|t| *t == t2).unwrap();
        let w: f64 = tokens[2].parse().unwrap();
        let d: f64 = tokens[3].parse().unwrap();
        let l: f64 = tokens[4].parse().unwrap();
        rates[i_t1][i_t2] = [w, d, l];
        rates[i_t2][i_t1] = [l, d, w];
    }

    let icount = count as i32;
    let matches: Vec<_> = (0..count)
        .into_iter()
        .flat_map(|t1| ((t1 + 1)..count).into_iter().map(move |t2| [t1, t2]))
        .collect();
    let mut scores = vec![0; count];
    let mut results = vec![0f64; count];
    dfs(icount, &rates, &matches, 1.0, &mut scores, &mut results);

    let mut output = String::new();
    for rate in results.iter() {
        writeln!(output, "{rate:.10}").unwrap();
    }
    println!("{output}");
}

fn dfs(
    count: i32,
    rates: &Vec<Vec<[f64; 3]>>,
    matches: &[[usize; 2]],
    rate: f64,
    scores: &mut [i32],
    results: &mut [f64],
) {
    if matches.is_empty() {
        let mut tickets = 2;
        for score in (0..=((count - 1) * 3)).rev() {
            if tickets <= 0 {
                break;
            }
            let mut candids: Vec<usize> = vec![];
            for (index, final_score) in scores.iter().enumerate() {
                if *final_score == score {
                    candids.push(index);
                }
            }
            if candids.len() <= tickets {
                for candid in candids.iter() {
                    results[*candid] += rate;
                }
                tickets -= candids.len();
            } else {
                let rate = rate / (candids.len() as f64) * (tickets as f64);
                for candid in candids.iter() {
                    results[*candid] += rate;
                }
                tickets = 0;
            }
        }
    } else {
        let [i, j] = matches[0];
        let next_ms = &matches[1..];
        scores[i] += 3;
        let r = rates[i][j][0];
        dfs(count, rates, next_ms, rate * r, scores, results);
        scores[i] -= 3;

        scores[i] += 1;
        scores[j] += 1;
        let r = rates[i][j][1];
        dfs(count, rates, next_ms, rate * r, scores, results);
        scores[i] -= 1;
        scores[j] -= 1;

        scores[j] += 3;
        let r = rates[i][j][2];
        dfs(count, rates, next_ms, rate * r, scores, results);
        scores[j] -= 3;
    }
}
