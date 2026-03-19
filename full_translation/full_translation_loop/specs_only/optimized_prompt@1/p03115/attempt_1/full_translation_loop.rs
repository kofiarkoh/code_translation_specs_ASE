use std::io::{self, BufRead};

const MAX_OUTPUT: i64 = 1_000_000_000; // Example constant, adjust as needed

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of coordinate pairs
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Ensure n is non-negative as per precondition
    assert!(n >= 0);

    let mut coordinates = Vec::with_capacity(n);

    // Read the coordinate pairs
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Ensure coordinates are non-negative as per precondition
        assert!(coords.len() == 2);
        assert!(coords[0] >= 0);
        assert!(coords[1] >= 0);

        coordinates.push((coords[0], coords[1]));
    }

    // Compute the result based on the coordinates
    let result = compute_result(&coordinates);

    // Ensure the result is within the predefined constant
    assert!(result <= MAX_OUTPUT);

    // Output the result
    println!("{}", result);
}

// Function to compute the result based on the coordinates
fn compute_result(coordinates: &Vec<(i64, i64)>) -> i64 {
    // Placeholder for combinatorial logic based on Lucas's Theorem
    // This should be replaced with the actual computation logic
    let mut result = 0;

    for &(x, y) in coordinates {
        // Example computation, replace with actual logic
        result += x + y; // Simple sum for demonstration
    }

    // Return the computed result
    result
}
