// top_three.rs
//
// Translated from the provided C program according to the SPEC.
// - Reads exactly 10 integer tokens from stdin (whitespace separated).
// - Maintains the three largest values seen so far in non-increasing order.
// - Initial ranks are {0,0,0} (so negative inputs will not be stored).
// - Uses strict '>' comparisons like the C original.
// Notes:
// - The original C used `scanf("%d\n", &work)` which has subtle whitespace-blocking behavior.
//   Here we read whitespace-separated tokens from stdin. This follows the SPEC (which assumes
//   10 valid integer tokens available) and preserves the core semantics.
// - As in the SPEC/C program, this code does not attempt robust recovery from invalid input:
//   parsing failures or fewer than 10 tokens will cause the program to panic (non-zero exit).
//   This mirrors the original program's assumption that the caller supplies 10 valid integers.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string, then split on whitespace into tokens.
    let mut s = String::new();
    io::stdin()
        .read_to_string(&mut s)
        .expect("failed to read stdin");

    let mut tokens = s.split_whitespace();

    // Ranks initialized to 0 as in the C code.
    let mut rank: [i32; 3] = [0, 0, 0];

    // Process exactly 10 integer tokens in sequence.
    for _ in 0..10 {
        // The SPEC/C program assumes valid integers are available and does not check scanf returns.
        // To follow that assumption, we unwrap/expect here: if input is invalid or too short, we will panic.
        let tok = tokens
            .next()
            .expect("expected at least 10 integer tokens on stdin");
        let work: i32 = tok.parse().expect("expected integer token");

        if work > rank[0] {
            rank[2] = rank[1];
            rank[1] = rank[0];
            rank[0] = work;
        } else if work > rank[1] {
            rank[2] = rank[1];
            rank[1] = work;
        } else if work > rank[2] {
            rank[2] = work;
        }
    }

    // Print the three ranks, one per line, as "%d\n" in C would.
    for i in 0..3 {
        println!("{}", rank[i]);
    }
}
