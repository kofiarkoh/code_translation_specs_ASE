// main.rs
//
// Translation of the provided specification into Rust.
//
// Notes on fidelity:
// - The original specification describes a program that allocates a global
//   Prime array for indices 0..MAX-1 (MAX = 1_000_000) and runs an
//   Eratosthenes sieve to mark primes. It also documents that the original
//   implementation contained out-of-bounds writes to Prime[MAX], which is
//   undefined behavior in C. To produce a safe, compilable Rust program while
//   staying faithful to the specification's intended semantics, this
//   implementation performs a bounds-safe sieve that guarantees Prime indices
//   0..MAX-1 reflect primality. Where the specification says behavior is
//   undefined (e.g., queries n >= MAX), this program chooses the narrowest
//   reasonable behavior: it treats such inputs as violations of preconditions
//   and will panic (which corresponds to an observable failure rather than
//   silently invoking undefined behavior).
//
// - Input parsing: the spec references scanf("%d", &n). This implementation
//   accepts signed integers separated by whitespace. Malformed tokens are
//   ignored (i.e., they are not parsed into an integer), which differs from
//   scanf's particular failure-to-consume semantics but is within the
//   specification's allowance that malformed input leads to unspecified
//   behavior. The core required observable behavior (for valid integer inputs
//   0..MAX-1) is preserved exactly.
//
// - The program prints, for each input integer n, the count of indices i in
//   0..=n whose Prime[i] is non-zero (1). For n < 0 it prints 0 as per the
//   specification's description of loop behavior. For n >= MAX it panics.

use std::io::{self, Read};

const MAX: usize = 1_000_000;

fn prime_search_eratos() -> Vec<u8> {
    // Create a vector of length MAX. Each entry is 0 or 1: 1 means "prime".
    // Using u8 to mirror a boolean-like array as described in the spec.
    let mut prime = vec![1u8; MAX];

    // By definition 0 and 1 are not prime (if they exist in range).
    if MAX > 0 {
        prime[0] = 0;
    }
    if MAX > 1 {
        prime[1] = 0;
    }

    // Sieve of Eratosthenes (bounds-safe).
    // Iterate i from 2 up to floor(sqrt(MAX-1)).
    let max_minus_one = MAX.saturating_sub(1);
    let limit = (max_minus_one as f64).sqrt() as usize;
    for i in 2..=limit {
        if prime[i] != 0 {
            // Start marking at i*i (fits in usize because i <= sqrt(MAX-1))
            let mut j = i * i;
            while j < MAX {
                prime[j] = 0;
                j += i;
            }
        }
    }

    prime
}

fn main() {
    // Precompute the sieve before processing queries.
    let prime = prime_search_eratos();

    // Read all stdin into a string, then parse whitespace-separated tokens as integers.
    // This is a practical and simple way to process one integer per token.
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        // If reading stdin fails, exit (mirrors abnormal termination).
        eprintln!("Failed to read stdin: {}", e);
        std::process::exit(1);
    }

    // Iterate tokens and attempt to parse as 32-bit signed integers (matches "%d" behavior).
    // For each successfully parsed integer, produce one output line.
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for token in input.split_whitespace() {
        // Attempt to parse token as i32. If parsing fails, skip the token.
        // The original C behavior with scanf("%d") on malformed input is
        // unspecified per the specification; skipping invalid tokens is a
        // narrow, reasonable interpretation that preserves behavior for valid
        // inputs.
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => {
                // Skip malformed tokens.
                continue;
            }
        };

        if n < 0 {
            // Per specification: loop for (i = 0; i <= n; ++i) does not execute when n < 0,
            // and the program prints 0.
            if let Err(_) = writeln!(out, "0") {
                // If writing fails, exit; this mirrors abnormal termination.
                std::process::exit(1);
            }
            continue;
        }

        let nu = n as usize;

        if nu >= MAX {
            // The specification states that n >= MAX leads to undefined behavior
            // (out-of-bounds access). To avoid performing actual undefined behavior
            // in Rust, we choose to treat this as a precondition violation and
            // panic. This makes the failure observable and avoids silent memory
            // corruption. This choice is the narrowest reasonable interpretation
            // that avoids inventing extra functionality.
            panic!("Query value n = {} out of allowed range [0, {}]. This is a precondition violation (would be undefined behavior in the original specification).", n, MAX - 1);
        }

        // Count primes in 0..=nu. Each prime[i] is 0 or 1.
        // Sum u8s into usize for printing.
        let count: usize = prime[..=nu].iter().map(|&b| b as usize).sum();

        if let Err(_) = writeln!(out, "{}", count) {
            std::process::exit(1);
        }
    }

    // Normal termination: main returns with exit code 0.
}
