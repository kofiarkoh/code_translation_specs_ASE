use std::io::{self, Write};

fn f(n: i64, k: i64) -> i64 {
    if n == 0 {
        return if k <= 0 { 0 } else { 1 };
    }
    let p = (1i64 << n) - 1; // Calculate the number of nodes in a complete binary tree of depth n
    if k <= 2 * p {
        f(n - 1, k - 1)
    } else {
        p + 1 + f(n - 1, k - 1 - 2 * p)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");
    
    // Ensure preconditions are met
    if n < 0 || k < 0 {
        eprintln!("n and k must be non-negative integers.");
        return;
    }

    let result = f(n, k);
    println!("{}", result);
}
