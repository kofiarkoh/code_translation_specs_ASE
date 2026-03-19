use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // behavior on exhaustion is unspecified; terminate quietly
        }
        let x_opt = it.next();
        if x_opt.is_none() {
            break; // behavior on missing value is unspecified; terminate quietly
        }

        let n: i64 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // malformed input behavior unspecified; terminate quietly
        };
        let x: i64 = match x_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n == 0 && x == 0 {
            break;
        }

        // Count unordered triples of distinct integers in [1, n] summing to x.
        // Unordered and distinct => count each triple exactly once by enforcing a < b < c.
        let mut count: i64 = 0;
        for a in 1..=n {
            for b in (a + 1)..=n {
                for c in (b + 1)..=n {
                    if a + b + c == x {
                        count += 1;
                    }
                }
            }
        }

        out.push_str(&count.to_string());
        out.push('\n');
    }

    print!("{out}");
}
