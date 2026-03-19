use std::io::{self, Write};
use std::fmt;

fn main() {
    let (n, k) = read_input();
    let result = compute_expression(n, k);
    println!("{:.12}", result);
}

// Function to read two integers from standard input
fn read_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("N must be a positive integer");
    let k: usize = iter.next().unwrap().parse().expect("K must be a non-negative integer");
    
    // Preconditions check
    assert!(n > 0, "N must be a positive integer");
    assert!(k >= 0, "K must be a non-negative integer");
    
    (n, k)
}

// Function to compute the mathematical expression based on N and K
fn compute_expression(n: usize, k: usize) -> f64 {
    let mut intermediate_values = vec![0.0; n]; // Array of size N
    let mut product = 1.0; // Initialized to 1

    // Compute the sum of ratios as per the specification
    for i in 0..n {
        intermediate_values[i] = (i + 1) as f64 / (k + 1) as f64; // Example ratio computation
        product *= intermediate_values[i]; // Update product
    }

    product // Return the computed result
}
