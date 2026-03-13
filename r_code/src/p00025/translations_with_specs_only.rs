// hits_and_blows.rs
//
// Reads repeated pairs of 4 integers from stdin and for each pair prints:
// "<hit> <blow>\n"
//
// Behavior notes (kept close to the Specification):
// - Input is expected as whitespace-separated decimal integers.
// - The program parses tokens sequentially as i32. On the first token that
//   fails to parse as an integer, parsing stops (subsequent tokens are not
//   considered). This choice matches a narrow, well-defined interpretation
//   of the C behavior where scanf matching failures lead to unspecified
//   behavior; here we stop further processing rather than inventing other behavior.
// - Only complete groups of 8 parsed integers (A[0..3], B[0..3]) produce output.
//   Any trailing incomplete group is ignored (no output for that partial case).
// - For each complete case, hit = count of i where A[i] == B[i].
//   blow = count of pairs (i,j) with i != j and A[i] == B[j].
// - Duplicate values are allowed and each matching index-pair is counted
//   independently (i.e., blows count every cross-index equal pair).
//
// This implementation is safe, idiomatic Rust and preserves the observable I/O
// behavior described in the Specification.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit quietly (no output). This is consistent with
        // treating stdin availability as a precondition; program returns non-error code by default.
        return;
    }

    // Parse tokens sequentially. On first parse failure, stop processing further tokens.
    let mut ints: Vec<i32> = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<i32>() {
            Ok(n) => ints.push(n),
            Err(_) => break, // stop on first non-integer token (narrow interpretation)
        }
    }

    // Process complete groups of 8 integers
    let mut i = 0usize;
    while i + 7 < ints.len() {
        let a = &ints[i..i + 4];
        let b = &ints[i + 4..i + 8];

        // compute hits
        let mut hit = 0;
        for idx in 0..4 {
            if a[idx] == b[idx] {
                hit += 1;
            }
        }

        // compute blows: count pairs (i,j) with i != j and a[i] == b[j]
        let mut blow = 0;
        for ia in 0..4 {
            for ib in 0..4 {
                if ia != ib && a[ia] == b[ib] {
                    blow += 1;
                }
            }
        }

        // Print exactly "<hit> <blow>\n"
        println!("{} {}", hit, blow);

        i += 8;
    }
}
