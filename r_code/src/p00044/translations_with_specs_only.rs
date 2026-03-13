// primes_bounds.rs
//
// Implements the behavior described in the Specification:
// - Precompute a fixed table of primes in ascending order that are < 60000,
//   and append a sentinel value 60000.
// - Read whitespace-separated integers (interpreted as C `int`) from stdin
//   until EOF. For each integer `a`:
//     - Find the first stored prime >= a.
//     - If that stored prime equals a (i.e. a is in the table), print the
//       previous and next primes around that stored prime.
//     - Otherwise, print the largest prime < a and the smallest prime > a.
// - For intended inputs (3 <= a < 60000) behavior matches the specification.
// - For out-of-range inputs the original specification marks behavior as
//   undefined; this implementation avoids panics by making narrow, simple
//   fallbacks (documented below).
//
// Notes on choices made to stay faithful to the specification:
// - The prime table is computed at program start (sieve) and contains all
//   primes < 60000, followed by the sentinel 60000.
// - Per-query search uses a linear scan from the start (not binary search),
//   matching the described linear-scan behavior.
// - If the first prime in the table (2) is the first >= a (i.e. a <= 2),
//   the specification states behavior is undefined; to keep the program safe
//   and not crash, we print the first two entries in the table as a narrow
//   fallback. For valid inputs (a >= 3) this branch is not taken.

use std::io::{self, Read};

fn compute_primes_below(limit: usize) -> Vec<i32> {
    // Sieve of Eratosthenes to compute primes < limit.
    // limit is expected to be 60000 in this program.
    let mut is_prime = vec![true; limit];
    if limit > 0 {
        is_prime[0] = false;
    }
    if limit > 1 {
        is_prime[1] = false;
    }
    let mut p = 2usize;
    while p * p < limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple < limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }
    let mut primes: Vec<i32> = Vec::new();
    for i in 0..limit {
        if is_prime[i] {
            primes.push(i as i32);
        }
    }
    primes
}

fn main() {
    // Prepare prime table: all primes < 60000, then sentinel 60000.
    const LIMIT: usize = 60000;
    let mut primes = compute_primes_below(LIMIT);
    // Ensure ascending sequence starts with 2 (sieve guarantees it if LIMIT > 2)
    // Append sentinel 60000 as specified.
    primes.push(60000);

    // Read entire stdin and split on whitespace to get tokens similar to scanf("%d")
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, just exit normally (no outputs).
        return;
    }

    for token in input.split_whitespace() {
        // Interpret each token as a C int -> use i32.
        // The specification assumes tokens are valid integers; if parsing fails,
        // we skip the token (narrow, safe choice).
        let a: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Linear scan from start to find first stored prime >= a
        let mut idx: usize = 0;
        while idx < primes.len() && primes[idx] < a {
            idx += 1;
        }

        // Now idx is the first index with primes[idx] >= a, or idx == primes.len()
        // (which should not happen for intended inputs because sentinel 60000 exists).
        if idx >= primes.len() {
            // Out-of-range case (a > sentinel). Specification marks behavior undefined.
            // Narrow fallback: print the last two elements we have.
            if primes.len() >= 2 {
                let n = primes.len();
                println!("{} {}", primes[n - 2], primes[n - 1]);
            } else if primes.len() == 1 {
                println!("{} {}", primes[0], primes[0]);
            } else {
                // No primes at all: print nothing (shouldn't occur).
            }
            continue;
        }

        if primes[idx] == a {
            // a matches a stored prime: print previous and next primes around it.
            // For intended inputs (a >= 3) idx >= 1 and idx+1 < primes.len() hold.
            // For edge cases (e.g., a == 2), idx == 0 and idx-1 would be invalid.
            // To avoid panics, use narrow fallback when idx == 0.
            if idx == 0 {
                // Undefined per spec; fallback to first two entries.
                if primes.len() >= 2 {
                    println!("{} {}", primes[0], primes[1]);
                } else if primes.len() == 1 {
                    println!("{} {}", primes[0], primes[0]);
                }
            } else {
                let prev = primes[idx - 1];
                // idx+1 is safe because we have appended sentinel 60000.
                let next = if idx + 1 < primes.len() {
                    primes[idx + 1]
                } else {
                    // Shouldn't happen because sentinel is appended, but be defensive.
                    primes[idx]
                };
                println!("{} {}", prev, next);
            }
        } else {
            // primes[idx] > a: print largest prime < a (idx-1) and smallest prime > a (idx).
            if idx == 0 {
                // This means a < first prime (2). Spec marks behavior undefined.
                // Narrow fallback: print first two entries.
                if primes.len() >= 2 {
                    println!("{} {}", primes[0], primes[1]);
                } else if primes.len() == 1 {
                    println!("{} {}", primes[0], primes[0]);
                }
            } else {
                let lower = primes[idx - 1];
                let higher = primes[idx];
                println!("{} {}", lower, higher);
            }
        }
    }
}
