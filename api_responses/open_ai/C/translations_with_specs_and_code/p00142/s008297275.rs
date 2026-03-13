// quadres_dist.rs
//
// Translated from the provided C program according to the SPEC.
// Reads integers n from stdin until a 0 is encountered. For each positive n,
// computes the distinct quadratic residues { i^2 mod n | i = 1..n-1 } (duplicates removed),
// then counts ordered pairs of distinct residues by their minimal cyclic distance
// d = min((r1-r2) mod n, (r2-r1) mod n) for d = 1..floor((n-1)/2).
//
// Behavior and I/O formatting follow the SPEC. For n <= 0 (except 0 which terminates),
// behavior is unspecified by the SPEC; this implementation simply skips negative n values
// (safe choice to avoid undefined/unsafe behavior).

use std::io::{self, Read};

fn main() {
    // Read all input tokens as integers
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, exit quietly (mirrors unspecified behavior on I/O failure).
        return;
    }

    let mut tokens = input.split_whitespace();
    while let Some(tok) = tokens.next() {
        // Parse integer; if parse fails, stop processing further (similar to scanf failing).
        let n: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n == 0 {
            break; // termination signal
        }

        if n < 0 {
            // SPEC: behavior for negative n is unspecified. To remain safe, skip.
            continue;
        }

        let n_usize = n as usize;

        // h = floor((n-1)/2)
        let h = ((n - 1) / 2) as usize;

        if n_usize <= 1 {
            // For n == 1, h == 0 and nothing should be printed.
            // Continue to next input.
            continue;
        }

        // Build distinct quadratic residues in order of first occurrence (like the C code).
        let mut seen = vec![false; n_usize];
        let mut residues: Vec<usize> = Vec::new();

        for i in 1..n_usize {
            // compute i*i % n using 64-bit temporaries to avoid any overflow issues
            let ii = (i as i64) * (i as i64);
            let r = (ii % (n as i64)) as usize;
            if !seen[r] {
                seen[r] = true;
                residues.push(r);
            }
        }

        // counts[d] will hold counts for distance d, 0..=h. index 0 unused.
        let mut counts = vec![0usize; h + 1];

        // Count ordered pairs of distinct residues.
        for &r1 in &residues {
            for &r2 in &residues {
                if r1 != r2 {
                    // compute (r1 - r2) mod n in 0..n-1
                    let mut d = if r1 >= r2 {
                        r1 - r2
                    } else {
                        r1 + n_usize - r2
                    };
                    // minimal cyclic distance
                    if d > h {
                        d = n_usize - d;
                    }
                    // d is in 1..=h
                    if d >= 1 && d <= h {
                        counts[d] += 1;
                    } else {
                        // For safety: if something unexpected occurs (shouldn't), ignore.
                    }
                }
            }
        }

        // Print counts for d = 1..=h, each on its own line, no extra blank lines.
        for d in 1..=h {
            println!("{}", counts[d]);
        }
    }
}
