use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Specification: reads four non-negative integers A, B, C, X.
    // If fewer are provided, treat as absent; unwrap() matches typical contest-style behavior.
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let c: i64 = it.next().unwrap().parse().unwrap();
    let x: i64 = it.next().unwrap().parse().unwrap();

    // Count combinations of three non-negative integers within [0..A], [0..B], [0..C]
    // such that 500*i + 100*j + 50*k == X (the "specified weights").
    // Use i64 to avoid overflow for typical ranges; count is non-negative.
    let mut count: i64 = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
