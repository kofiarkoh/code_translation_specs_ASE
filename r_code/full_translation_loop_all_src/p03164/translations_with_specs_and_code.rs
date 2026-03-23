use std::io::{self, Read};

const MEW: usize = 112345; // supports value totals 0..=112344

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    // Input read failure => nonzero exit, no guaranteed output
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(x) => x,
        None => std::process::exit(1),
    };
    let w_limit: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(x) => x,
        None => std::process::exit(1),
    };

    let mut w = vec![0i32; n];
    let mut v = vec![0usize; n];
    for i in 0..n {
        w[i] = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(x) => x,
            None => std::process::exit(1),
        };
        v[i] = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(x) => x,
            None => std::process::exit(1),
        };
    }

    // DP: ayaneru[val] = minimum achievable weight for total value `val`,
    // or sentinel (w_limit + 1) meaning "not achievable within tracked bounds".
    let sentinel = w_limit + 1;
    let mut ayaneru = vec![sentinel; MEW];
    ayaneru[0] = 0;

    for i in 0..n {
        let vi = v[i];
        let wi = w[i];
        // Iterate backwards to ensure each item is used at most once.
        for j in (vi..MEW).rev() {
            let prev = ayaneru[j - vi];
            let cand = prev + wi;
            if cand < ayaneru[j] {
                ayaneru[j] = cand;
            }
        }
    }

    for val in (0..MEW).rev() {
        if ayaneru[val] <= w_limit {
            print!("{val}\n");
            return;
        }
    }

    // Per spec: should not occur normally since value 0 is always achievable.
    print!("error!\n");
}
