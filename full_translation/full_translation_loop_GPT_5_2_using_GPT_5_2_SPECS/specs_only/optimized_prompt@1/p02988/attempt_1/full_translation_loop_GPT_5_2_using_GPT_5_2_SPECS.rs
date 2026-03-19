use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();

    // Fixed-size storage for up to 1000 integers, as specified.
    // If n exceeds capacity, behavior is unspecified by the specification; we do not add checks.
    let mut a = vec![0i64; 1000];

    // Read exactly n integers into storage (1-based indexing in spec; we store 0-based).
    // If input is insufficient/invalid, behavior is unspecified; unwrap() matches "no explicit error handling".
    for i in 0..(n as usize) {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    let mut count: i64 = 0;

    if n >= 3 {
        for i in 0..((n - 2) as usize) {
            let x = a[i];
            let y = a[i + 1];
            let z = a[i + 2];
            if (x < y && y < z) || (x > y && y > z) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
