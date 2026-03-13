// Translated from the provided C program.
// Notes on semantic choices:
// - The original C used `int` for both `i` and `j`. We use i32 to match a typical 32-bit `int`.
// - C's signed integer overflow is undefined, but many implementations behave as two's-complement wraparound.
//   To emulate the common behavior and preserve arithmetic wraparound, we use wrapping_mul.
// - The C program used scanf("%d",&i) without checking its return value. If parsing fails in Rust,
//   we treat that as 0 to avoid undefined/uninitialized behavior in safe Rust. This choice is documented
//   because the original C would have undefined behavior on invalid input.
//
// I/O formatting: prints the integer followed by a single newline, matching printf("%d\n", j).

use std::io::{self, Read};

fn main() {
    // Read all stdin and take the first whitespace-separated token as the integer,
    // similar to scanf("%d", &i) which reads the next integer token.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    // Parse first token as i32. If parsing fails (no input or invalid), treat as 0.
    let mut i: i32 = input
        .split_whitespace()
        .next()
        .and_then(|tok| tok.parse::<i32>().ok())
        .unwrap_or(0);

    let mut j: i32 = 1;

    // for (; i>0; i--) { j *= i; }
    while i > 0 {
        // Use wrapping_mul to emulate typical two's-complement wraparound behavior.
        j = j.wrapping_mul(i);
        i -= 1;
    }

    // Print result with a single trailing newline, like printf("%d\n", j);
    println!("{}", j);
}
