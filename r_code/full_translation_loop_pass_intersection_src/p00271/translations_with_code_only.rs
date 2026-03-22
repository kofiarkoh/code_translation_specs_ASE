use std::io::{self, Read};

fn main() {
    // Read entire stdin, like repeated scanf calls would consume tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    for _ in 0..7 {
        // The C code does not check scanf's return value; if input is missing,
        // behavior is undefined. Here we unwrap to fail similarly (panic).
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();

        // In C, signed overflow is undefined. Rust's wrapping_sub provides a
        // deterministic two's-complement wraparound, which is a common intended behavior.
        let diff = a.wrapping_sub(b);
        println!("{}", diff);
    }
}
