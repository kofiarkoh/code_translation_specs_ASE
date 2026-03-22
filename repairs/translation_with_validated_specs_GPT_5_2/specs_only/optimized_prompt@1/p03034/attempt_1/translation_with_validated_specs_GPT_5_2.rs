use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_opt = it.next();
    if n_opt.is_none() {
        // No input: unspecified by spec; do nothing.
        return;
    }
    let n: usize = n_opt.unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(tok) = it.next() {
            a.push(tok.parse::<i64>().unwrap());
        } else {
            // Insufficient input: unspecified by spec; stop early.
            break;
        }
    }

    let mut ans: i64 = 0;

    // Consider step sizes d with 1 <= d and 2d < n-1.
    // If none exist, ans remains 0.
    if n >= 3 {
        let mut d: usize = 1;
        while 2 * d < n - 1 {
            let mut s: usize = 0;
            let mut g: usize = n - 1;
            let mut crr: i64 = 0;

            loop {
                let ns = s + d;
                let ng = g - d;

                // Terminate before performing an addition if any condition holds.
                if ng <= d || ns == ng || ns == g {
                    break;
                }

                s = ns;
                g = ng;

                crr += a[s] + a[g];
                if crr > ans {
                    ans = crr;
                }
            }

            d += 1;
        }
    }

    // Output is at least 0 by construction.
    println!("{}", ans);
}
