use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Specification states arithmetic completes without overflow; use normal ops.
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;

    // Invariant: max_so_far is always >= each considered result.
    let mut max_so_far = sum;
    if diff > max_so_far {
        max_so_far = diff;
    }
    if prod > max_so_far {
        max_so_far = prod;
    }

    print!("{}", max_so_far);
}
