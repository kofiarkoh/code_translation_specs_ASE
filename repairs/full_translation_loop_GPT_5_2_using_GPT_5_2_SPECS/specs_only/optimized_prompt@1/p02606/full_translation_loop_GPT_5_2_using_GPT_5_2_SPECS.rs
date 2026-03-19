use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let l: i128 = it.next().unwrap().parse().unwrap();
    let u: i128 = it.next().unwrap().parse().unwrap();
    let d: i128 = it.next().unwrap().parse().unwrap();

    // Preconditions (per spec): d != 0, l <= u, and l,u are non-negative.
    // Use truncating integer division semantics consistent with Rust for non-negative values.
    let count = u / d - (l - 1) / d;

    println!("{count}");
}
