// translate_weighted_permutations.rs
//
// Translated from the provided C program according to the SPEC.
// This program precomputes counts dp[n][s] = number of permutations of digits 0..9
// whose first n digits yield weighted sum s = sum_{i=1..n} digit_i * i.
// Then it reads pairs of integers (n, s) from stdin until EOF and prints the
// corresponding count followed by a newline. For n > 10 or s > 330 it prints 0.
// Behavior for negative n or s is left undefined (matching the SPEC/C program).

use std::io::{self, Read};

fn solve(k: usize, s: usize, num: &mut [i32; 10], dp: &mut [[u32; 331]; 11]) {
    for i in k..10 {
        num.swap(i, k);

        // compute new sum: note positions are 1-based so weight is (k+1)
        let ts = s + (num[k] as usize) * (k + 1);
        dp[k + 1][ts] = dp[k + 1][ts].saturating_add(1);

        if k < 9 {
            solve(k + 1, ts, num, dp);
        }

        num.swap(i, k);
    }
}

fn main() {
    // dp[n][s] for n = 0..10, s = 0..330
    let mut dp: [[u32; 331]; 11] = [[0u32; 331]; 11];

    // initial ordering matches the C code: {1,2,3,4,5,6,7,8,9,0}
    let mut num: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    // Precompute counts
    solve(0, 0, &mut num, &mut dp);

    // Read all input, parse whitespace-separated tokens, and process pairs.
    // Stop if a token fails to parse as an integer or if an incomplete pair remains.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, nothing to do.
        return;
    }

    let mut it = input.split_whitespace();

    loop {
        let tn = match it.next() {
            Some(tok) => match tok.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break, // malformed token: stop processing
            },
            None => break, // EOF
        };

        let ts = match it.next() {
            Some(tok) => match tok.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break, // malformed token: stop processing
            },
            None => break, // incomplete pair: stop processing
        };

        // Match C behavior: if n <= 10 && s <= 330 print dp[n][s], else print 0.
        // Note: as per SPEC, negative n or s is undefined. Casting negative to usize will
        // cause an out-of-bounds access (panic) in Rust; this mirrors the undefined nature.
        if tn <= 10 && ts <= 330 {
            let idx_n = tn as usize;
            let idx_s = ts as usize;
            println!("{}", dp[idx_n][idx_s]);
        } else {
            println!("0");
        }
    }
}
