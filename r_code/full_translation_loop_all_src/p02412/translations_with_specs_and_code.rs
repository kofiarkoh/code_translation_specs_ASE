use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as integers (matches C behavior: no special invalid-input handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        let x_opt = it.next();
        if n_opt.is_none() || x_opt.is_none() {
            break; // No more complete pairs; C would block, but for batch input this is fine.
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let x: i32 = x_opt.unwrap().parse().unwrap();

        if n == 0 && x == 0 {
            break;
        }

        let mut b: i32 = 0;
        for k in 1..=n {
            for j in 1..=n {
                for i in 1..=n {
                    if i == j || j == k || i == k {
                        continue;
                    }
                    if i + j + k == x {
                        b += 1;
                    }
                }
            }
        }

        // Divide by 6 to convert ordered triples into unique combinations.
        out.push_str(&(b / 6).to_string());
        out.push('\n');
    }

    print!("{out}");
}
