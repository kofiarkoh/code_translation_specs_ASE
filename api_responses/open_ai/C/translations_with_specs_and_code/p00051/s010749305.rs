// kaprekar8.rs
//
// Translated from the provided C program according to the SPEC.
// Note about a subtle mismatch between the original C and the SPEC:
// - The C program extracts digits using `m % 10` and `m /= 10` on a signed int.
//   Behavior for negative `m` with `%` and `/` on signed integers is implementation-dependent
//   in C and is unspecified by the SPEC. The SPEC assumes non-negative `m` and that
//   digits considered are in 0..9. To preserve the SPEC invariants (digits 0..9),
//   this Rust translation uses the absolute value of `m` when extracting decimal digits.
//   This differs from the original C behavior on negative inputs; when `m` is negative
//   the C program could produce implementation-dependent results, while this program
//   will operate on the magnitude of `m`. This choice is deliberate to follow the SPEC.
//   For other aspects (parsing tokens, lack of validation), the translation mirrors the
//   original program's unspecified failure behavior by using unwrap() on parsing/token access.
//
// I/O behavior: reads whitespace-separated integers from stdin:
//   first token: n (number of cases). Then n tokens follow, each an integer m.
// For each m, extracts the least-significant 8 decimal digits (padding with leading zeros),
// forms the ascending and descending 8-digit numbers, computes descending - ascending,
// and prints the result followed by a newline. If n <= 0, prints nothing.
//
// On malformed or insufficient input tokens this program will panic (matching the C
// program's lack of scanf-result validation as unspecified behavior).

use std::io::Read;

fn main() {
    // Read all stdin
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    // Tokenize by whitespace. Use unwrap() on parsing to mirror unspecified behavior on malformed input.
    let mut toks = s.split_whitespace();

    // Read n
    let n_str = toks.next().unwrap();
    let n: i64 = n_str.parse().unwrap();

    // If n <= 0, do nothing (no output lines), as per SPEC.
    if n <= 0 {
        return;
    }

    let count = n as usize;

    for _ in 0..count {
        let m_str = toks.next().unwrap();
        let m: i64 = m_str.parse().unwrap();

        // Work with magnitude to ensure digits are in 0..9 (see note above).
        // Use i128 to safely handle potential edge cases like MIN.
        let mut mm = m as i128;
        if mm < 0 {
            mm = -mm;
        }

        // Extract least-significant 8 decimal digits (with zero padding).
        let mut digits = [0u8; 8];
        for j in 0..8 {
            digits[j] = (mm % 10) as u8;
            mm /= 10;
        }

        // Sort ascending
        let mut dv = digits.to_vec();
        dv.sort_unstable(); // ascending

        // Build min (ascending order digits -> most significant first is dv[0])
        let mut min: i32 = 0;
        for j in 0..8 {
            min += dv[j] as i32;
            if j < 7 {
                min *= 10;
            }
        }

        // Build max (descending order)
        let mut max: i32 = 0;
        for j in (0..8).rev() {
            max += dv[j] as i32;
            if j > 0 {
                max *= 10;
            }
        }

        let diff = max - min;
        println!("{}", diff);
    }
}
