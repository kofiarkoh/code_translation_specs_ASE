// transcribed from the provided C program according to the SPEC.
// Behavior: read signed integers from stdin (whitespace-separated) until EOF.
// For each integer s, compute:
//   ans = sum_{a=0..min(1000,s)} sum_{b=0..min(1000,s-a)} (501 - |500 - (s - a - b)|)
// and print ans followed by a newline.
//
// Notes on translation choices (kept short per instructions):
// - Inputs are parsed as i32 to match C's scanf("%d",&s) semantics for token ranges.
// - The accumulator is i64 to avoid accidental runtime overflow/panics in Rust debug builds.
//   The original C used a C int and would exhibit undefined behavior on overflow; the SPEC
//   documents that overflow is not handled. Using i64 preserves the mathematically correct
//   result for all reasonable inputs and avoids undefined behavior in Rust.
// - We assume tokens are valid integers per SPEC. Invalid (non-integer) tokens are ignored
//   (this is a pragmatic choice — the original C program would get stuck on a non-integer token).
// - Output formatting is identical: one decimal integer per line.

use std::io::{self, Read};

fn main() {
    // Read entire stdin to a string, then split on whitespace tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If a read error occurs, mimic standard behavior: just exit.
        return;
    }

    for token in input.split_whitespace() {
        // Try to parse token as i32, similar to scanf("%d",&s).
        // If parse fails, skip the token (SPEC assumes tokens are valid integers).
        let s_i32 = match token.parse::<i32>() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Promote to i64 for accumulation to avoid Rust panics on overflow and to
        // produce the mathematically intended sum.
        let s = s_i32 as i64;
        let mut ans: i64 = 0;

        // Loop a from 0..=1000 but stop early if a > s.
        for a in 0..=1000i64 {
            if s < a {
                break;
            }
            for b in 0..=1000i64 {
                if s < a + b {
                    break;
                }
                let c = s - a - b;
                let weight = 501 - (500 - c).abs();
                ans += weight;
            }
        }

        println!("{}", ans);
    }
}
