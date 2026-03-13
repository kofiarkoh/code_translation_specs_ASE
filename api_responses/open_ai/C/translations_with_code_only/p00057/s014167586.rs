// translated from the provided C program
// - Uses i32 to represent C's `int`.
// - Uses wrapping_add to emulate two's-complement wraparound behavior on overflow,
//   matching the practical behavior the original C program relies on.
//
// I/O behavior: reads whitespace-separated tokens from stdin and attempts to parse
// each as an i32. On a parse failure we stop (similar to encountering EOF / non-number).

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    // Read all stdin (scanf in a loop consumes tokens until EOF).
    io::stdin().read_to_string(&mut input).unwrap();

    for token in input.split_whitespace() {
        let a: i32 = match token.parse() {
            Ok(v) => v,
            // If a token isn't a valid integer, stop processing (closest practical mapping).
            Err(_) => break,
        };

        let mut b: i32 = 0;
        // c runs from 1 to a inclusive. If a < 1 the loop is skipped, matching the C code.
        for c in 1_i32..=a {
            // Use wrapping_add to preserve C-like wraparound on overflow.
            b = b.wrapping_add(c);
        }

        // Print b + 1, using wrapping_add to match potential overflow behavior.
        println!("{}", b.wrapping_add(1));
    }
}
