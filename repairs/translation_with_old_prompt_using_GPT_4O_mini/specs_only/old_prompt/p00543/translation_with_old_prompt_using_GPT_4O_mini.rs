use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = params.next().unwrap().parse().expect("Failed to parse m");

    // Read the array a[]
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let value: i32 = line.parse().expect("Failed to parse array element");
        a.push(value);
    }

    // Sort the array based on the modulo operation with divisors from 1 to m
    for k in 1..=m {
        a.sort_by_key(|&x| (x % k as i32 + k as i32) % k as i32); // Ensure non-negative modulo
    }

    // Output the sorted array
    for value in a {
        println!("{}", value);
    }
}
