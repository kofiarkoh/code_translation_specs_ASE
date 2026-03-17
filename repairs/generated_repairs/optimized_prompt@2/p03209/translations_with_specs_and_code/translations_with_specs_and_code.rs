use std::io::{self, BufRead};

fn f(n: i64, x: i64) -> i64 {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    }
    let p = (1i64 << n) - 1; // Calculate 2^n - 1
    if x <= 2 * p {
        f(n - 1, x - 1)
    } else {
        p + 1 + f(n - 1, x - 1 - 2 * p)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Keep as mutable
    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().expect("Missing n").parse().expect("Invalid n");
    let k: i64 = iter.next().expect("Missing k").parse().expect("Invalid k");

    // Preconditions: n and k must be non-negative integers
    assert!(n >= 0, "n must be non-negative");
    assert!(k >= 0, "k must be non-negative");

    let result = f(n, k);
    println!("{}", result);
}
