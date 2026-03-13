// primes_count.rs
//
// Translation of the given specification into Rust.
// - Builds a primality table for indices 0..MAX-1 (one byte per index).
// - Reads whitespace-separated signed 32-bit integers from stdin until EOF.
// - For each integer n:
//     - If n < 0: prints 0.
//     - Otherwise: counts primes for indices 0..=n and prints that count (one line).
// - For n >= MAX the original C specification declares behavior undefined.
//   To stay faithful to that, this implementation does not guard against n >= MAX;
//   indexing past bounds will cause a runtime panic, which mirrors a crash/erroneous behavior.
// - Uses safe Rust (no unsafe blocks). The out-of-range behavior therefore manifests as a panic
//   rather than C-style undefined memory corruption.

use std::io::{self, Read};

const MAX: usize = 1_000_000;

fn build_sieve() -> Vec<u8> {
    // One byte per index: 1 => prime, 0 => not prime
    let mut is_prime = vec![1u8; MAX];

    if MAX > 0 {
        is_prime[0] = 0;
    }
    if MAX > 1 {
        is_prime[1] = 0;
    }

    // Sieve of Eratosthenes
    let limit = (MAX as f64).sqrt() as usize + 1;
    for p in 2..=limit {
        if p >= MAX {
            break;
        }
        if is_prime[p] == 1 {
            let start = p.checked_mul(p).unwrap_or(MAX); // p*p; if overflow, treat as >= MAX
            if start >= MAX {
                continue;
            }
            for multiple in (start..MAX).step_by(p) {
                is_prime[multiple] = 0;
            }
        }
    }

    is_prime
}

fn main() {
    // Build primality table before processing queries (required by spec).
    let is_prime = build_sieve();

    // Read entire stdin into a string, then parse whitespace-separated tokens as i32.
    // The specification assumes valid integer tokens; if parsing fails we stop processing.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If stdin cannot be read for some reason, exit (process terminates with code 0 as normal).
        return;
    }

    for token in input.split_whitespace() {
        // Parse as signed 32-bit integer, like scanf("%d").
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => {
                // The specification states behavior on invalid tokens is unspecified.
                // Choose the narrowest reasonable action: stop processing further tokens.
                break;
            }
        };

        if n < 0 {
            println!("0");
            continue;
        }

        // For n >= MAX the specification says behavior is undefined.
        // To remain faithful we do not clamp or guard against out-of-range values here.
        // Indexing past bounds will cause a runtime panic in Rust (a form of crash).
        let limit = n as usize;

        // Count primes in [0, n] inclusive by scanning the primality table.
        let mut count: usize = 0;
        for i in 0..=limit {
            // This indexing will panic if limit >= MAX, reflecting undefined/out-of-bounds behavior.
            count += is_prime[i] as usize;
        }

        println!("{}", count);
    }
}
