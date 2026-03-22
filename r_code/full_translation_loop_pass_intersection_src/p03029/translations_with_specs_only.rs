use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let whole: i128 = it.next().unwrap().parse().unwrap();
    let half: i128 = it.next().unwrap().parse().unwrap();

    // Compute: (3 * whole + half) / 2 with truncation toward zero (Rust integer division).
    // Preconditions state no overflow; using i128 provides a wide range while preserving semantics.
    let result = (3 * whole + half) / 2;

    println!("{result}");
}
