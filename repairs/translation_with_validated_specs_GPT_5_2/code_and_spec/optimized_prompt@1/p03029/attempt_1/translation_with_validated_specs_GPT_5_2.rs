use std::io::{self, Read};

fn main() {
    // Read entire stdin (behavior unspecified on invalid/missing input, per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let p: i32 = it.next().unwrap().parse().unwrap();

    // Matches C semantics for representable intermediate values (per preconditions).
    let result = (a * 3 + p) / 2;

    println!("{result}");
}
