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

// Function to read input values for N and K
fn read_input() -> (u32, u32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input and parse N and K
    let mut iter = input.split_whitespace();
    let n: u32 = iter.next().expect("N is required").parse().expect("N must be a positive integer");
    let k: u32 = iter.next().expect("K is required").parse().expect("K must be a non-negative integer");

    // Preconditions: N must be positive and K must be non-negative
    if n == 0 {
        panic!("N must be a positive integer");
    }
    if k < 0 {
        panic!("K must be a non-negative integer");
    }

    (n, k)
}

// Function to compute the mathematical expression
fn compute_expression(n: u32, k: u32) -> f64 {
    // Example mathematical operation: (N + K) / (N - K + 1)
    // Ensure the denominator is non-zero
    let denominator = (n as i32) - (k as i32) + 1; // Cast to i32 to allow for negative values
    if denominator == 0 {
        panic!("Denominator must not be zero");
    }

    // Perform the computation and return the result as f64
    (n as f64 + k as f64) / (denominator as f64)
}
