use std::io::{self, BufRead};

fn main() {
    // Read input values
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().expect("Failed to read line").expect("Invalid input");
    
    let values: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly 6 inputs
    if values.len() != 6 {
        eprintln!("Expected exactly 6 long long integers as input");
        return; // Exit gracefully instead of panicking
    }

    // Unpack the input values
    let (t1, t2, a1, a2, b1, b2) = (values[0], values[1], values[2], values[3], values[4], values[5]);

    // Calculate the linear combinations
    let lhs = a1 * t1 + a2 * t2;
    let rhs = b1 * t1 + b2 * t2;

    // Check for equality and compute the result
    if lhs == rhs {
        // If the linear combinations are equal, output "infinity"
        println!("infinity");
    } else {
        // Otherwise, compute the difference
        let result = lhs - rhs;
        println!("{}", result);
    }
}
