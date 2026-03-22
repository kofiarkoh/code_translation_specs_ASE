use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Specification precondition states no overflow/underflow occurs for:
    // a - 2*b, so normal i64 arithmetic is sufficient.
    let expr = a - 2 * b;
    let out = if expr > 0 { expr } else { 0 };

    print!("{}", out);
}
