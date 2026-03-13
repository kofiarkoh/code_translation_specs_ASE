// translated from the provided C program.
// Behavior notes:
// - Uses i32 and wrapping_add to emulate typical two's-complement wrap-around
//   semantics that C implementations commonly exhibit on signed overflow.
// - Reads pairs of integers from stdin separated by any whitespace, like scanf("%d %d").
// - For each pair prints the number of bytes in the decimal representation of (a+b),
//   equivalent to the strlen of the sprintf-ed buffer in the C program.

use std::io::{self, Read};

fn main() {
    // Read all stdin (scanf in a loop consumes whitespace-separated tokens;
    // reading whole input and splitting on whitespace preserves that behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut toks = input.split_whitespace();

    loop {
        // Attempt to get two tokens, like scanf("%d %d") expects two conversions.
        let a_tok = match toks.next() {
            Some(t) => t,
            None => break, // EOF
        };
        let b_tok = match toks.next() {
            Some(t) => t,
            None => break, // EOF or incomplete pair
        };

        // Parse tokens as i32. If parsing fails, mimic scanf failing to match and stop.
        let a = match a_tok.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let b = match b_tok.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Use wrapping_add to emulate typical C implementation behavior for signed overflow.
        let sum = a.wrapping_add(b);

        // In the C code, sprintf(buf, "%d", a+b); printf("%d\n", strlen(buf));
        // The length is simply the ASCII byte length of the decimal representation.
        let s = sum.to_string();
        let len = s.len();

        println!("{}", len);
    }
}
