// primes_sum.rs
//
// Translated from the provided C program according to the SPEC.
// - Reads integers from stdin one-by-one.
// - For each nonzero d: prints the sum of the first d primes followed by a newline.
// - Stops when a zero is read.
// - For d <= 0 the behavior follows the original program's branch for d <= 2:
//   negative values yield 0 (sum over zero elements).
//
// Notes about differences / translation choices:
// - The original C used a fixed-size buffer for up to 1000 primes and exhibited
//   undefined behavior for d > 1000. To avoid undefined behavior in Rust and
//   produce well-defined results, this translation uses a dynamically-growing
//   Vec to store discovered primes. This makes the program safe for d > 1000.
//   The SPEC lists d in 1..1000 as the intended supported range; this
//   implementation preserves semantics for that range and extends behavior
//   safely for larger d.
// - Implementation follows the same trial-division approach as the C code:
//   test candidate odd numbers by dividing by known primes up to sqrt(candidate).

use std::io::{self, Read};

fn main() {
    // Read all input from stdin and iterate over whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (the SPEC does not require explicit handling).
        return;
    }

    for token in input.split_whitespace() {
        // Parse token as 32-bit signed integer (as per SPEC).
        let d: i32 = match token.parse() {
            Ok(x) => x,
            Err(_) => {
                // SPEC treats non-integer tokens as undefined behavior; stop processing.
                break;
            }
        };

        if d == 0 {
            // Termination sentinel: stop processing and exit normally.
            break;
        }

        // Compute and print sum for this d.
        let sum = sum_first_primes(d);
        println!("{}", sum);
    }
}

/// Compute the sum of the first `d` prime numbers, following the original
/// program's control flow for small d values:
/// - If d <= 0: returns 0 (matches the original program's incidental behavior).
/// - If d == 1: returns 2.
/// - If d == 2: returns 2 + 3 = 5.
/// - For d > 2: uses trial division by known primes up to sqrt(candidate).
fn sum_first_primes(d: i32) -> i64 {
    if d <= 0 {
        return 0;
    }

    let d_usize = d as usize;

    // Seed primes as in the original C program: s[0]=2, s[1]=3.
    let mut primes: Vec<i64> = Vec::with_capacity(d_usize.max(2));
    primes.push(2);
    primes.push(3);

    if d_usize <= 2 {
        // Sum the first d primes directly.
        primes.iter().take(d_usize).copied().sum()
    } else {
        // Start sum as 2 + 3 = 5 (same as the C code).
        let mut sum: i64 = 5;
        let mut candidate: i64 = 5;

        // Find primes until we have d primes in total.
        while primes.len() < d_usize {
            let limit = (candidate as f64).sqrt() as i64;
            let mut is_composite = false;

            // Test divisibility by known primes up to sqrt(candidate).
            for &p in &primes {
                if p > limit {
                    break;
                }
                if candidate % p == 0 {
                    is_composite = true;
                    break;
                }
            }

            if !is_composite {
                primes.push(candidate);
                sum += candidate;
            }

            candidate += 2; // test only odd candidates (even > 2 not prime)
        }

        sum
    }
}
