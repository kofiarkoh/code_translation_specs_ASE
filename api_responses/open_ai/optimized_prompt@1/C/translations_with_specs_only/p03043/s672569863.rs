use std::io::{self, Write};
use std::fmt;

fn main() {
    // Read input values for N and K
    let (n, k) = read_input();

    // Compute the result based on the mathematical expression
    let result = compute_expression(n, k);

    // Print the result formatted to twelve decimal places
    println!("{:.12}", result);
}

// Function to read two integers from standard input
fn read_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string and parse the integers
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().expect("N is required").parse().expect("N must be a positive integer");
    let k: usize = iter.next().expect("K is required").parse().expect("K must be a non-negative integer");

    // Preconditions: N must be positive and K must be non-negative
    assert!(n > 0, "N must be a positive integer");
    assert!(k >= 0, "K must be a non-negative integer");

    (n, k)
}

// Function to compute the mathematical expression based on N and K
fn compute_expression(n: usize, k: usize) -> f64 {
    // Initialize an array to store intermediate values
    let mut values = vec![0.0; n];

    // Fill the array with computed ratios
    for i in 0..n {
        // Here we assume a specific mathematical operation for demonstration
        // The operation could be something like (i + 1) as a ratio
        values[i] = (i + 1) as f64 / (k + 1) as f64; // Example ratio
    }

    // Compute the sum of the ratios
    let sum: f64 = values.iter().sum();

    // Return the computed result
    sum
}
