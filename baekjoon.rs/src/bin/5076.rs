use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.trim().split('\n').collect();
    let mut stack: Vec<&str> = vec![];
    let mut results: Vec<bool> = vec![];
    'outer: for line in lines {
        stack.clear();
        if line == "#" {
            break;
        }
        let mut started = false;
        let mut start_index = 0_usize;
        let mut end_index = 0_usize;
        for (index, letter) in line.chars().enumerate() {
            if letter == '<' {
                started = true;
                start_index = index;
            } else if letter == '>' {
                started = false;
                end_index = index;

                let curr = &line[start_index..=end_index];
                if curr.starts_with("</") {
                    if let Some(prev) = stack.pop() {
                        let mut open = String::new();
                        for letter in prev.chars().skip(1) {
                            if letter == ' ' || letter == '>' {
                                break;
                            } else {
                                open.push(letter);
                            }
                        }
                        let mut close = String::new();
                        for letter in curr.chars().skip(2) {
                            if letter == ' ' || letter == '>' {
                                break;
                            } else {
                                close.push(letter);
                            }
                        }
                        if open != close {
                            results.push(false);
                            continue 'outer;
                        }
                    } else {
                        results.push(false);
                        continue 'outer;
                    }
                } else if !curr.ends_with("/>") {
                    stack.push(curr);
                }
            }
        }
        results.push(stack.is_empty());
    }

    let mut output = String::new();
    for result in results.iter() {
        if *result {
            writeln!(&mut output, "legal").unwrap();
        } else {
            writeln!(&mut output, "illegal").unwrap();
        }
    }
    println!("{output}");
}
