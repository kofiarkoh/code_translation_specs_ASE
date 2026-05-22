use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace separated parsing)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
    let b: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);

    // In C, signed overflow is undefined. We emulate typical two's-complement wraparound
    // to preserve likely intended behavior.
    let expr = a.wrapping_sub(b.wrapping_mul(2));
    let out = if expr > 0 { expr } else { 0 };

    // Match printf("%d", ...) with no trailing newline.
    print!("{}", out);
}
