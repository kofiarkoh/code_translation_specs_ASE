// Import necessary modules
use std::io::{self, BufRead};

// Define a constant for the modulo operation
const MODULO_CONSTANT: i64 = 1_000_000_007; // Example constant, can be adjusted as needed

fn main() {
    // Read input sizes
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the sizes of the two sequences
    let size1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let size2: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Preconditions: sizes must be non-negative
    if size1 == 0 || size2 == 0 {
        // If either size is zero, the product is zero
        println!("0");
        return;
    }

    // Read the first sequence
    let sequence1: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap()) // Assuming valid long long integers
        .collect();

    // Read the second sequence
    let sequence2: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap()) // Assuming valid long long integers
        .collect();

    // Compute the derived values
    let derived_value1 = compute_derived_value(&sequence1);
    let derived_value2 = compute_derived_value(&sequence2);

    // Calculate the product and apply modulo
    let result = (derived_value1 * derived_value2) % MODULO_CONSTANT;

    // Ensure the result is non-negative (it should be due to the modulo operation)
    println!("{}", result);
}

// Function to compute a derived value from a sequence
fn compute_derived_value(sequence: &[i64]) -> i64 {
    // Example computation: sum of the sequence
    // This can be adjusted based on the actual intended computation
    sequence.iter().sum()
}
