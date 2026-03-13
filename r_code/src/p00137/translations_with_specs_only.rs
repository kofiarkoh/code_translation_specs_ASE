// Translated from the provided specification.
// Behavior:
//  - Read an integer d (number of test cases) from stdin.
//  - For each of the next d integers (seeds), generate 10 iterates:
//      s_{n+1} = (s_n * s_n / 100) % 10000
//    and print them as specified.
//
// Notes on translation choices:
//  - The specification treats inputs as C signed int. We use i32 for that role.
//  - The specification requires that s*s fits in the signed int range. To avoid
//    accidental panics due to Rust overflow checks while preserving the intended
//    mathematical behavior, the square is computed in i64. This does not change
//    behavior for inputs that satisfy the precondition (s*s fits in i32).
//  - If input parsing fails or is insufficient, the specification says behavior is
//    unspecified; this program will simply stop processing further cases.
//  - If d <= 0, nothing is printed (per the precondition that d >= 0).
use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // I/O error reading stdin: behavior unspecified per spec; exit silently.
        return;
    }

    // Tokenize by whitespace
    let mut toks = input.split_whitespace();

    // Read d as C signed int equivalent (i32)
    let d: i32 = match toks.next().and_then(|t| t.parse::<i32>().ok()) {
        Some(v) => v,
        None => {
            // Missing or invalid d: unspecified by spec; exit silently.
            return;
        }
    };

    // Per specification, d must be non-negative. If it's <= 0, do nothing.
    if d <= 0 {
        return;
    }

    // Process up to d test cases. The spec assumes exactly d seeds follow;
    // if tokens run out, behavior is unspecified — we stop processing.
    for case_idx in 1..=(d as i32) {
        // Read seed s as i32 (C signed int equivalent)
        let s0_i32 = match toks.next().and_then(|t| t.parse::<i32>().ok()) {
            Some(v) => v,
            None => {
                // Insufficient/invalid input: unspecified by spec; stop.
                return;
            }
        };

        // Print header: "Case k:"
        println!("Case {}:", case_idx);

        // Use i64 for intermediate to avoid Rust overflow, consistent with spec's
        // requirement that s*s fits in signed int for valid inputs.
        let mut s = s0_i32 as i64;

        // Produce and print ten iterates s1..s10
        for _ in 0..10 {
            // Compute s = (s*s / 100) % 10000
            // s*s is >= 0 for all integer s; division truncates toward zero,
            // which is the same as floor for non-negative values.
            let sq = s.wrapping_mul(s); // wrapping_mul on i64 won't panic; inputs are safe per spec
            let div = sq / 100; // integer division
            let next = div % 10000; // result in 0..9999 for non-negative div
            // Ensure next is non-negative (div >= 0 so this holds; defensive cast)
            let next_nonneg = if next < 0 { next + 10000 } else { next };
            // Print the iterate as decimal integer on its own line
            println!("{}", next_nonneg);
            // Update s to next iterate for subsequent iteration
            s = next_nonneg;
        }

        // At this point, per spec, the in-memory s equals s10 (not observable)
    }
}
