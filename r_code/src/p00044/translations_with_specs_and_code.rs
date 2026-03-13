// primes_bounds.rs
//
// Translates the given C program (per the provided specification) into safe,
// idiomatic Rust. Behavior matches the specification for intended inputs:
//   - Reads whitespace-separated integers from stdin until EOF.
//   - For each integer `a` (intended 3 <= a < 60000), prints one line:
//       "<largest_prime_less_than_a> <smallest_prime_greater_than_a>\n"
//     If `a` itself is a prime from the precomputed table, prints the primes
//     immediately before and after `a`.
// Implementation notes:
//   - Builds the same prime table as the C code using trial division logic.
//   - Appends sentinel 60000 to the primes table.
//   - Uses a linear search to find the first table entry strictly greater than `a`
//     (mirrors the C program's linear scan).
//   - For inputs outside the intended range (e.g. a < 3), the original C program
//     exhibited undefined behavior. This Rust program avoids panics: if an input
//     would require an out-of-bounds access (which corresponds to the C UB case),
//     the input is silently skipped. This preserves safety while adhering to the
//     specification's statement that such inputs are undefined.
//
// No unsafe code is used.

use std::io::{self, Read};

fn main() {
    // Build prime table p[] with the same algorithm as the C program:
    // p[0] = 2; for a in 3..60000:
    //   test divisibility by primes p[b] for b=0..n-1
    //   if a % p[b] == 0 -> composite -> break
    //   if a / p[b] < p[b] -> no need to test further, a is prime -> push
    // Finally push sentinel 60000.
    let mut primes: Vec<i32> = Vec::new();
    primes.push(2);
    for a in 3..60000 {
        let mut added = false;
        for &pb in primes.iter() {
            if a % pb == 0 {
                // composite; stop checking this a
                added = false;
                break;
            }
            if a / pb < pb {
                // pb exceeded sqrt(a); a is prime
                primes.push(a);
                added = true;
                break;
            }
        }
        // Note: if the loop completed without hitting either branch it means
        // primes was empty (impossible here) or logic would continue — but
        // current structure matches the C code behavior.
        let _ = added; // keep for clarity; no-op
    }
    primes.push(60000);

    // Read all input from stdin and parse integers
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, just exit silently (no output), matching the
        // idea that stdin must be available per spec.
        return;
    }

    // Iterate over whitespace-separated tokens and parse as i32
    let mut out_lines: Vec<String> = Vec::new();
    for token in input.split_whitespace() {
        let a: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => {
                // The C program used scanf and would fail on non-integer tokens.
                // Spec says non-integer handling is not defined; we skip such tokens.
                continue;
            }
        };

        // Find first index n such that a < primes[n], mirroring the C while loop.
        // There's always at least the sentinel 60000, so position should exist.
        let pos_opt = primes.iter().position(|&p| a < p);
        let n = match pos_opt {
            Some(idx) => idx,
            None => {
                // Should not happen because of sentinel, but skip defensively.
                continue;
            }
        };

        // For intended valid inputs (3 <= a < 60000) we expect n >= 2 so that
        // accesses to n-2 are valid when a equals a stored prime.
        if n < 2 {
            // This corresponds to inputs like a < 2 which the original C code
            // would have had undefined behaviour. To keep the Rust program safe,
            // we skip producing output for such inputs.
            continue;
        }

        if primes[n - 1] == a {
            // a matches a stored prime: print previous and next around that prime
            out_lines.push(format!("{} {}", primes[n - 2], primes[n]));
        } else {
            // a not a stored prime: print largest prime < a and smallest prime > a
            out_lines.push(format!("{} {}", primes[n - 1], primes[n]));
        }
    }

    // Print all output lines, each terminated by a newline.
    // This ensures exact stdout formatting: "<int> <int>\n" per input.
    if !out_lines.is_empty() {
        // Join with newlines and add a trailing newline to match println behavior.
        let mut all = out_lines.join("\n");
        all.push('\n');
        print!("{}", all);
    }
}
