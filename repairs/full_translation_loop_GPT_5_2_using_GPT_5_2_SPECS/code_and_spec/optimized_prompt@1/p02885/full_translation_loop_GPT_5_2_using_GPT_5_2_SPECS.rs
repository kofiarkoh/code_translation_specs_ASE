use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style; no extra output).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    // Spec/C: output max(0, a - 2*b).
    // Note: In C, signed overflow is undefined. In Rust, debug builds panic on overflow.
    // We use wrapping arithmetic to avoid introducing a panic and to keep behavior
    // unspecified-but-non-panicking, closer to C's "undefined" nature.
    let expr = a.wrapping_sub(b.wrapping_mul(2));
    let out = if expr > 0 { expr } else { 0 };

    print!("{out}");
}
