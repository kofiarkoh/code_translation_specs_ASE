use std::io::{self, Read};

const MAX_SIEVE: usize = 5000;

/// Prepare a simple sieve of Eratosthenes up to MAX_SIEVE (inclusive).
fn make_sieve() -> Vec<bool> {
    let mut is_prime = vec![true; MAX_SIEVE + 1];
    if MAX_SIEVE >= 0 {
        is_prime[0] = false;
    }
    if MAX_SIEVE >= 1 {
        is_prime[1] = false;
    }
    let mut p = 2usize;
    while p * p <= MAX_SIEVE {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= MAX_SIEVE {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }
    is_prime
}

fn main() {
    // Build primality table up to 5000 as per specification.
    let is_prime = make_sieve();

    // Read all whitespace-separated tokens from stdin.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, abort quietly (behavior on I/O failure is unspecified in the spec).
        return;
    }

    // Split by whitespace and parse integers. The spec treats input as 32-bit signed ints.
    for token in input.split_whitespace() {
        // If parsing fails, break out (spec: behavior on malformed input is unspecified).
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: stop when a 0 is encountered; 0 is not processed.
        if n == 0 {
            break;
        }

        // For each non-zero n, compute number of unordered prime pairs (p, q) with p + q == n.
        // According to the specification, correctness is only guaranteed for 0 <= n <= 5000
        // because primality information is only prepared up to 5000. To avoid panics,
        // we only consider primes p in [2, min(n, MAX_SIEVE)] and only treat n-p as prime
        // if n-p is within [0, MAX_SIEVE]. This may produce incorrect results for n > 5000,
        // which matches the stated limitation.
        let mut count_unordered = 0i32;

        if n >= 0 {
            let n_usize = n as usize;
            let upper = usize::min(n_usize, MAX_SIEVE);
            for p in 2..=upper {
                if !is_prime[p] {
                    continue;
                }
                // q = n - p; ensure q is within [0, MAX_SIEVE] before indexing.
                if n_usize < p {
                    // since q would be negative, skip
                    continue;
                }
                let q = n_usize - p;
                if q > MAX_SIEVE {
                    // We don't have primality info for q; per spec, results for n > MAX_SIEVE are not guaranteed.
                    continue;
                }
                if !is_prime[q] {
                    continue;
                }
                // Count unordered pairs: only count when p <= q so that (p,q) and (q,p) are not double-counted.
                if p <= q {
                    count_unordered += 1;
                }
            }
        } else {
            // Negative n: per specification, treat as having zero prime pairs (no primes <= negative numbers).
            count_unordered = 0;
        }

        // Print one line per processed input (non-zero).
        println!("{}", count_unordered);
    }
}
