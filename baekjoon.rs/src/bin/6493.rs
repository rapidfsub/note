use std::fmt::Write;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    buf.clear();

    let mut result: Vec<bool> = vec![];
    for _ in 0..n {
        stdin().read_line(&mut buf).unwrap();
        let line = buf.trim();
        let mut is_slurpy = false;
        for (index, _) in line.chars().enumerate().skip(1) {
            let front = &line[0..index];
            if is_slimp(front) {
                let back = &line[index..];
                if is_slump(back) {
                    is_slurpy = true;
                    break;
                }
            }
        }
        result.push(is_slurpy);

        buf.clear();
    }

    let mut output = String::new();
    for is_slurpy in result.iter() {
        if *is_slurpy {
            writeln!(&mut output, "YES").unwrap();
        } else {
            writeln!(&mut output, "NO").unwrap();
        }
    }
    println!("{output}");
}

fn is_slump(text: &str) -> bool {
    if text.starts_with('D') || text.starts_with('E') {
        if text.ends_with('G') {
            let mut f_count = 0;
            let mut start_index = 0;
            for (index, letter) in text.chars().enumerate().skip(1) {
                if letter == 'F' {
                    f_count += 1;
                } else {
                    start_index = index;
                    break;
                }
            }
            let next = &text[start_index..];
            f_count > 0 && (next == "G" || is_slump(&text[start_index..]))
        } else {
            false
        }
    } else {
        false
    }
}

fn is_slimp(text: &str) -> bool {
    if text.ends_with('C') {
        if let Some(slimp) = text.strip_prefix("AB") {
            is_slimp(slimp) || is_slimp(&slimp[..slimp.len() - 1])
        } else if let Some(slump) = text.strip_prefix('A').and_then(|x| x.strip_suffix('C')) {
            is_slump(slump)
        } else {
            false
        }
    } else {
        text == "AH"
    }
}
