// twin_primes.rs
//
// Translates the provided specification into Rust.
// Notes:
// - A global primality table `PRIME` is used, where 0 means "prime" and 1 means "composite".
// - setprime(max) fills the table for indices 0..=max (must be <= MAX).
// - The program reads integers from stdin until a zero is encountered.
// - For each nonzero n it searches downwards from n to 7 for the largest twin-prime pair (p, q) with q <= n and q - p = 2,
//   printing "p q\n" when found.
// - To avoid out-of-bounds accesses, queries larger than MAX are clamped to MAX (per the specification note that larger
//   values will not produce correct results beyond the precomputed range).
//
// Unsafe usage:
// - The primality table is declared as `static mut`. Accesses are confined to small, documented unsafe blocks inside
//   `setprime` (writes) and the query loop (reads). This mirrors the original program's global writable array while
//   keeping unsafety minimal and well-scoped.

use std::io::{self, Read};

const MAX: usize = 10000;

// Global primality table: 0 = prime, nonzero = composite.
// Use static mut to match the "global array" requirement from the specification.
// Accesses are wrapped in unsafe blocks where necessary.
static mut PRIME: [u8; MAX + 1] = [0; MAX + 1];

/// Precompute primality up to `max` (inclusive).
/// `max` must be <= MAX to avoid out-of-bounds behavior.
fn setprime(max: usize) {
    // Safety: we are the only code that mutates PRIME and we ensure max <= MAX.
    unsafe {
        // Initialize range 0..=max to 0 (prime) first.
        for i in 0..=max {
            PRIME[i] = 0;
        }

        // 0 and 1 are not prime; mark as composite (nonzero).
        if max >= 0 {
            PRIME[0] = 1;
        }
        if max >= 1 {
            PRIME[1] = 1;
        }

        let mut i: usize = 2;
        while i * i <= max {
            if PRIME[i] == 0 {
                // mark multiples of i as composite
                let mut j = i * i;
                while j <= max {
                    PRIME[j] = 1;
                    j += i;
                }
            }
            i += 1;
        }
    }
}

fn main() {
    // Precompute primes up to MAX as required by the specification.
    setprime(MAX);

    // Read all input tokens (like repeated scanf("%d",&n) calls).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, behave like unspecified in original: just exit.
        return;
    }

    for token in input.split_whitespace() {
        // Parse each token as a signed 32-bit integer.
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => {
                // The specification leaves non-integer tokens unspecified.
                // We choose to stop processing further tokens on parse error.
                break;
            }
        };

        // Zero terminates processing.
        if n == 0 {
            break;
        }

        // Only consider queries >= 7; for n < 7 the program prints nothing for that query.
        if n < 7 {
            continue;
        }

        // Clamp queries larger than MAX to MAX to avoid out-of-bounds access.
        let mut nn: usize = if n as i64 > MAX as i64 {
            MAX
        } else {
            n as usize
        };

        // Search downward from nn to 7 for the largest i such that i and i-2 are prime.
        let mut found = false;
        while nn >= 7 {
            let is_prime_i: bool;
            let is_prime_i_minus_2: bool;
            // Read from the global PRIME table (safe only inside unsafe).
            unsafe {
                is_prime_i = PRIME[nn] == 0;
                is_prime_i_minus_2 = PRIME[nn - 2] == 0;
            }
            if is_prime_i && is_prime_i_minus_2 {
                // Found twin primes (nn-2, nn)
                println!("{} {}", nn - 2, nn);
                found = true;
                break;
            }
            nn -= 1;
        }

        // If not found, the specification requires printing nothing for that query.
        let _ = found; // kept to reflect that we considered the possibility; no-op here.
    }

    // Normal termination (Rust exits with code 0 on successful completion).
}
