// Import necessary libraries
use std::io::{self, BufRead};

// Function to determine if a number is prime
fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Function to count integers satisfying the multiplicative condition based on prime factors
fn count_integers(max_value: usize) -> Vec<usize> {
    let mut count = vec![0; max_value + 1];

    for i in 1..=max_value {
        // Check if the number is prime
        if is_prime(i) {
            // Increment count for prime numbers
            count[i] = count[i - 1] + 1;
        } else {
            // For composite numbers, we can define a condition based on prime factors
            // Here we simply carry forward the previous count
            count[i] = count[i - 1];
        }
    }
    count
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read number of queries
    let q: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Failed to read Q"),
        _ => {
            eprintln!("Error reading number of queries.");
            std::process::exit(1);
        }
    };

    // Read the array of integers N
    let mut n_values = Vec::new();
    for _ in 0..q {
        let n: usize = match lines.next() {
            Some(Ok(line)) => line.trim().parse().expect("Failed to read N"),
            _ => {
                eprintln!("Error reading query value.");
                std::process::exit(1);
            }
        };
        n_values.push(n);
    }

    // Determine the maximum value in N
    let max_value = *n_values.iter().max().unwrap();

    // Compute the count of integers satisfying the condition
    let count = count_integers(max_value);

    // Output results for each query
    for &n in &n_values {
        println!("{}", count[n]);
    }
}
