use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-like behavior: whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let p: i32 = it.next().unwrap().parse().unwrap();

    // In C, signed overflow is undefined; typical contest inputs avoid it.
    // Use wrapping ops to avoid Rust panics in debug and to emulate common 2's-complement behavior.
    let result = a.wrapping_mul(3).wrapping_add(p) / 2;

    print!("{result}\n");
}
