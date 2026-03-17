use std::io::{self, BufRead};

fn max(x: usize, y: usize) -> usize {
    if x > y { x } else { y }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut iter = lines.iter();

    while let Some(line) = iter.next() {
        let mut line_iter = line.split_whitespace();
        let w: usize = match line_iter.next() {
            Some(num) => num.parse().unwrap_or(0),
            None => break,
        };
        let d: usize = match line_iter.next() {
            Some(num) => num.parse().unwrap_or(0),
            None => break,
        };

        if w == 0 {
            break;
        }

        let mut a = [0; 21];
        let mut b = [0; 21];

        for _ in 0..w {
            let t: usize = match iter.next() {
                Some(line) => line.trim().parse().unwrap_or(0),
                None => break,
            };
            if t < a.len() {
                a[t] += 1;
            }
        }

        for _ in 0..d {
            let t: usize = match iter.next() {
                Some(line) => line.trim().parse().unwrap_or(0),
                None => break,
            };
            if t < b.len() {
                b[t] += 1;
            }
        }

        let mut res = 0;
        for i in 1..=20 {
            res += i * max(a[i], b[i]);
        }

        println!("{}", res);
    }
}
