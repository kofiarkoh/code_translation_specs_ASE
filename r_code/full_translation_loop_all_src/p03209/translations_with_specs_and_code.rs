use std::io::{self, BufRead};

fn f(n: u64, x: u64) -> u64 {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    }
    let p = (1u64 << n) - 1; // Calculate 2^n - 1
    if x <= 2 * p {
        f(n - 1, x - 1)
    } else {
        p + 1 + f(n - 1, x - 1 - 2 * p)
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let n: u64 = iter.next().expect("Missing n").parse().expect("Invalid n");
    let k: u64 = iter.next().expect("Missing k").parse().expect("Invalid k");

    // Ensure preconditions are met
    assert!(n >= 0 && k >= 0, "Inputs must be non-negative integers");

    let result = f(n, k);
    println!("{}", result);
}
