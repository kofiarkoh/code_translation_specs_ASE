// translated from the provided C program
// Behavior notes:
// - Uses i64 to represent C 'long' (typical on modern 64-bit platforms).
// - Uses wrapping_add for increments of counters to emulate common two's-complement
//   wraparound behavior that some C implementations exhibit on overflow.
//   (C signed overflow is UB, but to avoid Rust panics in debug builds we wrap.)

use std::io::{self, Read};

fn main() {
    // Read all input, split on whitespace, parse as i64 tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    while let Some(tok) = iter.next() {
        // If parse fails, stop processing further (similar to scanf failing).
        let n: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }

        let mut a: i64 = 0;
        let mut i: i64 = 5;

        // Loop from i=5 to n inclusive. Use wrapping_add to avoid Rust debug-panics
        // on overflow and to loosely emulate typical C behavior.
        while i <= n {
            let mut b = i;
            while b > 0 {
                if b % 5 != 0 {
                    break;
                }
                a = a.wrapping_add(1);
                b = b / 5;
            }
            i = i.wrapping_add(1);
        }

        println!("{}", a);
    }
}
