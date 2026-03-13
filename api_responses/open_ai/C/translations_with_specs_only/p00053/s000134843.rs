use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens (assumed to be valid integers per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Parse tokens into i32. Per specification, non-integer tokens are undefined behavior;
    // here we simply unwrap, which will panic on invalid tokens (consistent with "not handled").
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Fixed-size buffer for primes (capacity 1000) to mirror the original program's design.
    // Accessing beyond this (i.e., for d > 1000) will panic at runtime in Rust, which
    // corresponds to the unspecified/undefined behavior noted in the specification.
    let mut primes: [i32; 1000] = [0; 1000];
    let mut count: usize = 0; // number of primes currently stored

    for d in tokens {
        if d == 0 {
            // Termination sentinel: stop processing and exit normally with code 0.
            return;
        }

        if d < 0 {
            // Specification: negative values are not explicitly validated and are
            // treated by the code path that typically results in printing 0.
            // Emulate that incidental behavior by printing 0 and continuing.
            println!("0");
            continue;
        }

        // d > 0: compute sum of first d primes.
        let required = d as usize;

        // Generate primes until we have 'required' primes.
        // This will panic if required > 1000 because we will attempt to write
        // beyond the fixed-size buffer, matching the specification's "unsupported" behavior.
        while count < required {
            let candidate: i32 = if count == 0 {
                2
            } else if count == 1 {
                // After 2, the next prime is 3; keeping candidate progression simple.
                3
            } else {
                // Start from last prime + 2 to keep odd candidates (since last prime >= 3).
                primes[count - 1] + 2
            };

            // For the general case when count >= 2, we may need to try multiple candidates.
            if count <= 1 {
                // Directly accept candidate (2 or 3).
                primes[count] = candidate;
                count += 1;
                continue;
            }

            // When count >= 2, we need to find the next prime by trial division.
            // We'll reuse a local mutable variable to try successive odd candidates.
            let mut cand = candidate as i64;
            loop {
                let mut is_prime = true;
                // Test divisibility by known primes up to sqrt(cand).
                let mut j = 0usize;
                while j < count {
                    let p = primes[j] as i64;
                    if p * p > cand {
                        break;
                    }
                    if cand % p == 0 {
                        is_prime = false;
                        break;
                    }
                    j += 1;
                }
                if is_prime {
                    // Safe to store; this may panic if count >= 1000 and we try to write out of bounds,
                    // which intentionally mirrors the "undefined" behavior for d > 1000 per spec.
                    primes[count] = cand as i32;
                    count += 1;
                    break;
                } else {
                    // Try next odd candidate.
                    cand += 2;
                }
            }
        }

        // Sum the first 'required' primes. Use i64 for the accumulator to avoid overflow for sums.
        let mut sum: i64 = 0;
        for i in 0..required {
            sum += primes[i] as i64;
        }

        // Print the sum followed by a newline.
        println!("{}", sum);
    }

    // If input ended without a terminating 0, we simply exit (spec leaves this undefined;
    // treating EOF as normal program termination is a narrow reasonable interpretation).
}
