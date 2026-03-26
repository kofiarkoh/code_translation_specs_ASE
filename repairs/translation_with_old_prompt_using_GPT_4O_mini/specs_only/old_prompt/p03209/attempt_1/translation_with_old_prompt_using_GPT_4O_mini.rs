use std::io::{self, Write};

fn f(n: u64, k: u64) -> u64 {
    // Base case: if n is 0, return 1 if k is greater than 0, otherwise return 0
    if n == 0 {
        return if k > 0 { 1 } else { 0 };
    }
    
    // Calculate the total number of nodes in a complete binary tree of depth n
    let total_nodes = (1 << n) - 1; // 2^n - 1

    // If k is greater than the total number of nodes, return 0
    if k > total_nodes {
        return 0;
    }

    // Recursive case: determine the value based on the position k
    let mid = (1 << (n - 1)); // 2^(n-1)
    if k < mid {
        // If k is in the left subtree
        f(n - 1, k)
    } else {
        // If k is in the right subtree
        f(n - 1, k - mid)
    }
}

fn main() {
    let mut input = String::new();
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<u64> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly two inputs
    if parts.len() != 2 {
        eprintln!("Invalid input. Please provide two non-negative integers.");
        return;
    }

    let n = parts[0];
    let k = parts[1];

    // Call the recursive function and store the result
    let result = f(n, k);

    // Output the result
    println!("{}", result);
}
