// primes_count.rs
//
// Translation of the provided C program (sieve + queries) to Rust,
// following the given SPEC as the source-of-truth.
//
// Notes about divergences / corrections:
// - The original C code's sieve used `for (j = i * 2; j <= MAX; j += i)` which writes
//   to Prime[MAX] (out-of-bounds). That conflicts with the SPEC which requires the
//   table to be indexed 0..MAX-1. This Rust implementation fixes that bug by using
//   the standard sieve inner loop starting at i*i and ensuring we only write indices
//   < MAX.
// - The SPEC states that queries n >= MAX are undefined. To avoid panics in Rust
//   while keeping behavior sensible for well-formed inputs (the SPEC requires clients
//   to provide n < MAX), this implementation clamps such queries to MAX-1 before
//   counting. This prevents runtime panics while otherwise preserving intended behavior
//   for valid inputs. For n < 0 the program prints 0 as specified.
//
// The program reads whitespace-separated integers from stdin until EOF. For each integer n,
// it prints the count of primes in [0, n] (one per line). MAX = 1_000_000.

use std::io::{self, Read};

const MAX: usize = 1_000_000;

fn prime_search_eratos() -> Vec<u8> {
    // Use a Vec<u8> with 1 = prime, 0 = not prime to mimic the original char array.
    let mut prime = vec![1u8; MAX];

    // 0 and 1 are not prime
    if MAX > 0 {
        prime[0] = 0;
    }
    if MAX > 1 {
        prime[1] = 0;
    }

    // Sieve up to sqrt(MAX). Use i*i as the inner loop start to avoid redundant work.
    let limit = (MAX as f64).sqrt() as usize;
    for i in 2..=limit {
        if prime[i] == 0 {
            continue;
        }
        // start at i*i to avoid marking lower multiples repeatedly
        let mut j = i * i;
        while j < MAX {
            prime[j] = 0;
            j += i;
        }
    }

    prime
}

fn main() {
    let prime = prime_search_eratos();

    // Read all input from stdin and split by whitespace into tokens.
    // This matches the behavior of scanf("%d") for well-formed inputs (whitespace-separated integers).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit early (no output). This mirrors environment-dependent behavior.
        return;
    }

    for token in input.split_whitespace() {
        // Try to parse each token as a 32-bit signed integer.
        // If parsing fails, skip the token. (C's scanf("%d") behavior on invalid tokens is unspecified;
        // skipping here avoids infinite loops and is a reasonable, safe choice.)
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if n < 0 {
            // For n < 0 the SPEC prescribes printing 0.
            println!("0");
            continue;
        }

        // For n >= MAX the SPEC declares behavior undefined. To avoid a runtime panic in Rust,
        // we clamp such queries to MAX-1 before counting. Clients should provide n < MAX.
        let mut idx = n as usize;
        if idx >= MAX {
            idx = MAX - 1;
        }

        // Count primes in [0, idx]
        let count: usize = prime[..=idx].iter().map(|&b| b as usize).sum();
        println!("{}", count);
    }
}
