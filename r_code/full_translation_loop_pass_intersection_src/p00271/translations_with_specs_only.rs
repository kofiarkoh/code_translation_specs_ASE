use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    for _ in 0..7 {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        // The specification states overflow/underflow is undefined; we use normal subtraction.
        // In debug builds, Rust may panic on overflow; in release builds it wraps.
        // The precondition says differences are representable, so overflow should not occur.
        let diff = a - b;

        println!("{}", diff);
    }
}
