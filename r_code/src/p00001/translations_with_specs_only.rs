// top_three.rs
//
// Translates the given Specification into Rust.
// Behavior notes (kept faithful to the Specification):
// - Reads exactly 10 integer tokens from stdin (whitespace-separated).
// - Assumes there are at least 10 valid integer tokens; does not handle parse failures.
// - Maintains three ranks initialized to 0 and updates them using strict '>' comparisons.
// - Prints the three ranks in non-increasing order, one per line, as decimal integers.
//
// Implementation choices:
// - Uses read_to_string + split_whitespace to obtain tokens. This requires the producer of
//   stdin to provide the tokens (typical for POSIX pipelines / redirection). The Specification
//   assumes valid tokens are available; behavior on EOF / invalid tokens is unspecified,
//   so this code uses unwrap/expect to reflect that (it will panic if tokens are missing or invalid).
// - Uses i32 to represent "C's int" for the purposes of this translation, per the Specification.

use std::io::{self, Read};

fn main() {
    // Initialize ranks as specified: rank = {0, 0, 0}
    let mut rank: [i32; 3] = [0, 0, 0];

    // Read all of stdin into a string and split on whitespace.
    // The Specification assumes inputs are available and valid; we therefore unwrap parses.
    let mut input = String::new();
    // read_to_string will block until EOF; this is acceptable under the spec's assumptions.
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let mut tokens = input.split_whitespace();

    for _ in 0..10 {
        // As per the Specification, the program assumes there are at least 10 valid integer tokens.
        // We follow that assumption and unwrap parse errors / missing tokens (panic on failure).
        let tok = tokens
            .next()
            .expect("expected at least 10 integer tokens on stdin");
        let work: i32 = tok.parse().expect("failed to parse integer token");

        // Update ranks using strict greater-than comparisons exactly as specified.
        if work > rank[0] {
            // shift down and insert at rank[0]
            rank[2] = rank[1];
            rank[1] = rank[0];
            rank[0] = work;
        } else if work > rank[1] {
            // shift rank[1] to rank[2], insert at rank[1]
            rank[2] = rank[1];
            rank[1] = work;
        } else if work > rank[2] {
            // insert at rank[2]
            rank[2] = work;
        } else {
            // no change
        }
    }

    // Print the three ranks, one per line, in non-increasing order.
    println!("{}", rank[0]);
    println!("{}", rank[1]);
    println!("{}", rank[2]);
}
