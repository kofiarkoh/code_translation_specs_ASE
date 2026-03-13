// boxing_weights.rs
//
// Translation of the given C program to Rust, following the provided specification.
// Behavior:
// - Reads whitespace-separated tokens from stdin.
// - For each token that successfully parses as an f64, classifies it according to the
//   specified thresholds (using <= comparisons) and prints the exact label followed by a newline.
// - If a token does not parse as an f64, this program stops processing further tokens.
//   NOTE: The original C program exhibited undefined behavior on non-numeric tokens
//   because it checked scanf(...) != EOF rather than checking the return value == 1.
//   The spec documents that as undefined behavior; this Rust translation chooses a
//   safe, well-defined behavior (stop on first non-numeric token) rather than trying
//   to emulate undefined/unsafe C behavior.
//
// The mapping (for any parsed numeric v) follows the specification exactly:
//  v <= 48            -> "light fly"
//  v <= 51            -> "fly"
//  v <= 54            -> "bantam"
//  v <= 57            -> "feather"
//  v <= 60            -> "light"
//  v <= 64            -> "light welter"
//  v <= 69            -> "welter"
//  v <= 75            -> "light middle"
//  v <= 81            -> "middle"
//  v <= 91            -> "light heavy"
//  else               -> "heavy"

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string, then split on whitespace tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we cannot read stdin, exit silently (no output), matching the spirit of the CLI tool.
        return;
    }

    for token in input.split_whitespace() {
        // Attempt to parse token as f64. If parsing fails, stop processing further tokens.
        // (This is a deliberate, safe choice. The original C program's behavior was undefined
        // for non-numeric tokens as documented in the specification.)
        let v: f64 = match token.parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        // Comparisons with NaN are false in Rust, so NaN will fall through to the final "heavy"
        // branch, matching the specification's description of the C program behavior.
        let label = if v <= 48.0 {
            "light fly"
        } else if v <= 51.0 {
            "fly"
        } else if v <= 54.0 {
            "bantam"
        } else if v <= 57.0 {
            "feather"
        } else if v <= 60.0 {
            "light"
        } else if v <= 64.0 {
            "light welter"
        } else if v <= 69.0 {
            "welter"
        } else if v <= 75.0 {
            "light middle"
        } else if v <= 81.0 {
            "middle"
        } else if v <= 91.0 {
            "light heavy"
        } else {
            "heavy"
        };

        println!("{}", label);
    }
}
