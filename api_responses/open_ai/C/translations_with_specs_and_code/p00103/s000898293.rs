// baseball.rs
//
// Translated from the provided C program according to the SPEC.
// Behavior:
//  - Read first token as integer `n` (number of segments).
//  - For each of n segments: process tokens "HIT", "HOMERUN", "OUT" until 3 outs.
//  - For each segment print the runs scored (one integer per line).
//
// Notes on translation choices:
//  - The SPEC is authoritative. The C code's behavior (e.g., using integer counts on bases
//    and while(n != 0)) is preserved to match semantics described in the SPEC.
//  - Input is tokenized by whitespace. If the input runs out before finishing a segment,
//    the program will stop processing further segments (this is safer than panicking).
//    The SPEC calls such situations undefined; the C program would have blocked on scanf
//    waiting for more input; here we consume all provided tokens first.
//  - No `unsafe` is used; all operations are safe and idiomatic Rust.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split into whitespace tokens.
    // This may differ from scanf's incremental blocking behavior on interactive stdin,
    // but it preserves functional behavior for typical non-interactive use.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (matches the C program which had no explicit error messages).
        return;
    }

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut idx = 0;

    // Parse the first token as an integer n. If missing or invalid, exit.
    let n: i32 = match tokens.get(idx).and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    idx += 1;

    // We intentionally mirror the C program's loop condition `while (n != 0)`.
    // For negative n this will not terminate in C; here we will mimic that behavior
    // logically by using the same arithmetic. However, because we consume tokens
    // deterministically from a finite vector, if tokens run out we'll stop.
    let mut n_remaining = n;
    while n_remaining != 0 {
        // Prepare segment state
        let mut score: i32 = 0;
        let mut base = [0i32; 3]; // base[0] = first, base[1] = second, base[2] = third
        let mut outs: i32 = 0;

        // Process events until 3 outs or until we run out of tokens.
        while outs < 3 {
            // If no more tokens available, stop processing further (safer than panic).
            let event = match tokens.get(idx) {
                Some(tok) => *tok,
                None => {
                    // Insufficient input; SPEC marks this as undefined. We stop processing.
                    return;
                }
            };
            idx += 1;

            if event == "HIT" {
                // Batter to first; then handle any over-occupancy by propagating one runner per base.
                base[0] += 1;
                for i in 0..3 {
                    if base[i] > 1 {
                        if i < 2 {
                            base[i + 1] += 1;
                        } else {
                            score += 1;
                        }
                        base[i] -= 1;
                    }
                }
            } else if event == "HOMERUN" {
                // All runners score plus the batter.
                for i in 0..3 {
                    if base[i] == 1 {
                        score += 1;
                        base[i] = 0;
                    }
                }
                score += 1; // batter scores
            } else if event == "OUT" {
                outs += 1;
            } else {
                // Invalid token: do nothing (per SPEC). Continue to next token.
            }
        }

        // After 3 outs, print the segment's score.
        println!("{}", score);

        // Decrement n_remaining just like the C code (`n--`).
        n_remaining -= 1;
    }
}
