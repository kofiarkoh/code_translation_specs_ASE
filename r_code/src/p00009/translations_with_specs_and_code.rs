// prime_count.rs
//
// Translated from the provided C program to Rust per the specification.
// Notes about differences / fixes (see SPEC):
// - The original C code wrote to Prime[MAX] (out-of-bounds) during sieve initialization.
//   That is undefined behavior. The SPEC describes the intended behavior: Prime should
//   represent primality for indices 0..MAX-1. This Rust implementation follows the
//   SPEC and fixes the out-of-bounds bug by only accessing indices 0..=MAX-1.
// - The original program used scanf("%d", &n). The SPEC assumes inputs are valid ints
//   in range 0..=MAX-1. This program reads whitespace-separated tokens from stdin and
//   parses them as i32. If parsing fails for a token, that token is skipped. If n < 0
//   we print 0 (matching the SPEC). If n >= MAX, behavior was unspecified in the SPEC;
//   here we treat it as an error and panic to avoid silent undefined behavior. This is
//   a deliberate, documented deviation to preserve safety in Rust.
//
// Behavior:
// - Precomputes sieve for MAX = 1_000_000 marking primes in range 0..=MAX-1.
// - For each integer token read from stdin, prints the count of indices i in 0..=n
//   for which Prime[i] is true, followed by a newline.
//
// Implementation notes:
// - Uses a Vec<u8> for the prime array to mirror the C char array semantics (0 or 1).
// - Sieve uses the standard i*i starting point for marking multiples for correctness
//   and efficiency.

use std::io::{self, Read};

const MAX: usize = 1_000_000;

fn prime_search_eratos() -> Vec<u8> {
    // Prime[i] == 1 means prime, 0 means not prime.
    let mut prime = vec![1u8; MAX];

    if MAX > 0 {
        prime[0] = 0;
    }
    if MAX > 1 {
        prime[1] = 0;
    }

    // Sieve: for i from 2 while i*i <= MAX-1
    let limit = ((MAX - 1) as f64).sqrt() as usize;
    for i in 2..=limit {
        if prime[i] == 0 {
            continue;
        }
        // Start from i*i to avoid redundant markings and stay within bounds.
        let mut j = i * i;
        while j < MAX {
            prime[j] = 0;
            j += i;
        }
    }

    prime
}

fn main() {
    // Precompute sieve once.
    let prime = prime_search_eratos();

    // Read entire stdin, parse whitespace-separated tokens as signed integers (i32).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (similar to C program which would likely fail).
        return;
    }

    for token in input.split_whitespace() {
        // Try parse as i32. If parse fails, skip the token.
        // The SPEC assumes well-formed integer input; skipping malformed tokens
        // is a safer choice than infinite loop or other undefined behavior.
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if n < 0 {
            // As per SPEC: if n < 0, the count loop does not run and the program prints 0.
            println!("0");
            continue;
        }

        let n_usize = n as usize;
        if n_usize >= MAX {
            // The SPEC states that n >= MAX is undefined behavior in the original C.
            // In Rust we avoid undefined behavior; here we panic to signal the error.
            // This is a documented deviation from the original program's UB.
            panic!("query value n out of range (must be 0..={})", MAX - 1);
        }

        // Count primes in 0..=n (inclusive). This is O(n) per SPEC.
        let mut count: usize = 0;
        for i in 0..=n_usize {
            if prime[i] != 0 {
                count += 1;
            }
        }

        println!("{}", count);
    }
}
