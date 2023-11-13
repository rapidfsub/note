use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut stack: Vec<&str> = vec![];
    let mut results: Vec<bool> = vec![];
    'outer: for line in input.trim().split('\n') {
        stack.clear();
        if line == "#" {
            break;
        }
        let mut start_index = 0_usize;
        for (index, letter) in line.chars().enumerate() {
            if letter == '<' {
                start_index = index;
            } else if letter == '>' {
                let curr = &line[start_index..=index];
                if curr.starts_with("</") {
                    if let Some(prev) = stack.pop() {
                        let open = get_tag_name(prev);
                        let close = get_tag_name(curr);
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

fn get_tag_name(tag: &str) -> String {
    let mut result = String::new();
    for letter in tag.chars().skip(1) {
        if letter == '<' || letter == '/' {
            continue;
        } else if letter == ' ' || letter == '>' {
            break;
        } else {
            result.push(letter);
        }
    }
    result
}
