// quadr_residues_dist.rs
//
// Translates the provided specification into Rust.
// Reads integers from stdin until a 0 is encountered. For each positive n (1 <= n <= 10000),
// computes the distinct quadratic residues { i^2 mod n | i = 1..n-1 } (duplicates removed)
// and prints h = floor((n-1)/2) lines: for d = 1..h the number of ordered pairs
// (r1, r2) of distinct residues whose minimal cyclic difference equals d.
//
// Note: The specification is ambiguous for even n regarding minimal difference n/2
// because h = floor((n-1)/2) excludes n/2. This implementation follows the spec literally:
// only d in 1..h are printed and only pairs whose minimal difference falls in that range
// are counted (pairs with minimal_diff == n/2 when n is even are therefore not reported).

use std::io::{self, Read};

fn main() {
    // Read entire stdin and split on whitespace.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, just exit silently as the original C program's behavior
        // on read failure is implementation-dependent. Exit with success code 0.
        return;
    }

    for token in input.split_whitespace() {
        // Parse each token as a signed 32-bit integer, as specified.
        let n_i32: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => {
                // If a token isn't an integer, skip it (narrow reasonable choice).
                continue;
            }
        };

        if n_i32 == 0 {
            // Termination signal: stop processing further inputs.
            break;
        }

        // The specification's intended domain is 1 <= n <= 10000.
        // For safety, ignore non-positive n values (behavior unspecified in spec).
        if n_i32 <= 0 {
            continue;
        }

        let n = n_i32 as usize;

        // Build set R of distinct quadratic residues r = (i*i) % n for i=1..n-1.
        let mut seen = vec![false; n];
        let mut residues: Vec<usize> = Vec::new();

        // i ranges from 1 to n-1 inclusive.
        for i in 1..n {
            let r = (i * i) % n;
            if !seen[r] {
                seen[r] = true;
                residues.push(r);
            }
        }

        // h = floor((n-1)/2)
        let h = (n.saturating_sub(1)) / 2;

        // Prepare counts for d = 1..h (index d-1).
        let mut counts = vec![0usize; h];

        // For every ordered pair of distinct residues, compute minimal cyclic difference.
        // diff = (r1 - r2) mod n in 0..n-1; minimal = min(diff, n - diff).
        // If minimal in 1..h, increment counts[minimal - 1].
        for &r1 in &residues {
            for &r2 in &residues {
                if r1 == r2 {
                    continue; // distinct residues only
                }
                // Compute diff in 0..n-1
                let diff = if r1 >= r2 { r1 - r2 } else { n + r1 - r2 };
                // minimal cyclic distance
                let minimal = std::cmp::min(diff, n - diff);
                if minimal >= 1 && minimal <= h {
                    counts[minimal - 1] += 1;
                }
                // If minimal == 0 it would mean r1 == r2 which we excluded.
                // If minimal > h (e.g., minimal == n/2 when n even), we do not count it
                // because the spec requires printing only d=1..h.
            }
        }

        // Print counts, one per line, in order d = 1..h. For h == 0 print nothing.
        for c in counts {
            println!("{}", c);
        }
    }
}
