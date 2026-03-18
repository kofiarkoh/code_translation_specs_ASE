use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Preconditions say input must contain at least one integer.
    // If missing, do nothing (narrowest behavior without inventing error handling).
    let n_opt = it.next();
    if n_opt.is_none() {
        return;
    }
    let n: usize = match n_opt.unwrap().parse() {
        Ok(v) => v,
        Err(_) => return, // Narrowest behavior on malformed input.
    };

    // O(n) space for tails; invariant: tails is strictly increasing.
    let mut tails: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        let x_str = match it.next() {
            Some(s) => s,
            None => break, // If fewer numbers than declared, process what we have.
        };
        let x: i64 = match x_str.parse() {
            Ok(v) => v,
            Err(_) => break, // Narrowest behavior on malformed input.
        };

        // Find the first index i where tails[i] >= x (lower_bound).
        // Replacing tails[i] with x preserves the invariant that tails is increasing.
        let mut lo: usize = 0;
        let mut hi: usize = tails.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if tails[mid] < x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        if lo == tails.len() {
            tails.push(x);
        } else {
            tails[lo] = x;
        }
    }

    // Output length of LIS (non-negative).
    println!("{}", tails.len());
}
