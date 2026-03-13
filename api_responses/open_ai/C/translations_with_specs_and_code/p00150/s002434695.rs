// primes_twin.rs
//
// Translation of the provided C program to Rust following the SPEC.
// Notes:
// - The original C code declared `char prime[10001];` (indices 0..10000) but called
//   `setprime(10001)`, which would index out-of-bounds. This Rust translation
//   follows the SPEC and ensures the precomputation bound does not exceed 10000.
//   We therefore precompute up to MAX = 10000 (inclusive). This preserves the
//   intended, defined behavior.
//
// Behavior preserved:
// - Precompute primality/composite markers in a global-like structure before queries.
//   Here `0` means "prime" and `1` means "composite" to match the original program's usage.
// - Read integers from stdin until a zero is read. For each nonzero n, find the largest
//   twin-prime pair (p, q) with q <= n (and q - p == 2) and print "p q\n".
// - If n > MAX, results are only valid up to MAX (we cap the search at MAX).
//
// Implementation notes:
// - Safe, idiomatic Rust; no `unsafe` used.
// - Input is read from stdin as whitespace-separated tokens and parsed as signed 32-bit integers.
// - Non-integer tokens are ignored (they simply fail to parse) and do not produce output.
// - This implementation prints nothing for queries that do not yield a twin-prime pair
//   (e.g., n < 7 or no twin primes <= n within the precomputed range).

use std::io::{self, Read};

const MAX: usize = 10000;

fn setprime(max: usize) -> Vec<u8> {
    // Ensure we do not exceed MAX to avoid out-of-bounds behavior.
    let max = if max > MAX { MAX } else { max };

    // prime[i] == 0 => i is considered prime
    // prime[i] == 1 => i is considered composite
    let mut prime = vec![0u8; max + 1];

    // Mark even numbers >= 4 as composite
    if max >= 4 {
        for i in (4..=max).step_by(2) {
            prime[i] = 1;
        }
    }

    // Sieve-like marking for odd primes
    let mut i = 3usize;
    while i * i <= max {
        if prime[i] == 0 {
            let mut j = i * 2; // matches the original C implementation (starts at i*2)
            while j <= max {
                prime[j] = 1;
                j += i;
            }
        }
        i += 2;
    }

    prime
}

fn main() {
    // Precompute primes up to MAX. (In the original C code this was intended but
    // mistakenly passed 10001; we use MAX to adhere to the SPEC and avoid UB.)
    let prime = setprime(MAX);

    // Read all stdin, split on whitespace, parse integers
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit normally (no output).
        return;
    }

    for token in input.split_whitespace() {
        // Try parsing as i32; skip tokens that don't parse as integers.
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if n == 0 {
            break; // zero terminates processing
        }

        // Cap search start at MAX to avoid out-of-bounds access; this follows the SPEC.
        if n < 7 {
            continue; // nothing is printed for n < 7
        }

        let mut start = n as isize;
        if start as usize > MAX {
            start = MAX as isize;
        }

        // If start < 7 after capping, nothing to do.
        if start < 7 {
            continue;
        }

        let mut found = false;
        let mut i = start as usize;
        while i >= 7 {
            // Safe indexing because i and i-2 are within 0..=MAX
            if prime[i] == 0 && prime[i - 2] == 0 {
                println!("{} {}", i - 2, i);
                found = true;
                break;
            }
            i -= 1;
        }

        // If no twin prime pair found, print nothing for this query (per SPEC).
        if !found {
            // continue to next token
        }
    }
}
