use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style; no extra prompts).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    // Per spec: overflow behavior is unspecified; we use normal i32 arithmetic.
    let c = a + b;
    let d = a - b;
    let e = a * b;

    // Spec requires max of (sum, difference, product).
    // Note: C code initializes max = -1000, which would be wrong if all results < -1000.
    // We follow the specification instead.
    let max_val = c.max(d).max(e);

    print!("{max_val}");
}
