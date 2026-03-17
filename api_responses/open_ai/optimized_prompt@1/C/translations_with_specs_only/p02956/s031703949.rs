// Import necessary libraries
use std::cmp::Ordering;

// Define a constant for the modulo operation
const MOD: i64 = 998244353;

// Define a struct to hold the key and identifier pairs
#[derive(Debug)]
struct Pair {
    key: i64,
    identifier: i64,
}

// Implement the main functionality
fn main() {
    // Read the number of pairs
    let n: usize = read_input(); // Assume read_input() reads a positive integer from input
    let mut pairs: Vec<Pair> = Vec::with_capacity(n);

    // Read the pairs of integers
    for _ in 0..n {
        let (key, identifier) = read_pair(); // Assume read_pair() reads a pair of integers from input
        pairs.push(Pair { key, identifier });
    }

    // Sort the pairs based on the key
    pairs.sort_by(|a, b| a.key.cmp(&b.key));

    // Initialize the result
    let mut result: i64 = 0;

    // Process the pairs to compute the result
    for pair in pairs {
        // Example computation: result += (key * identifier) % MOD
        // This is a placeholder for the actual computation logic
        result = (result + (pair.key * pair.identifier) % MOD) % MOD;
    }

    // Output the final result
    println!("{}", result);
}

// Placeholder function to read an integer from input
fn read_input() -> usize {
    // Implementation for reading input goes here
    // For now, returning a dummy value
    5
}

// Placeholder function to read a pair of integers from input
fn read_pair() -> (i64, i64) {
    // Implementation for reading a pair of integers goes here
    // For now, returning dummy values
    (1, 2)
}
