use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line of input
        let line = line.expect("Failed to read line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok()) // Parse integers, ignoring invalid inputs
            .collect();

        // Ensure we have exactly two integers
        if nums.len() != 2 {
            continue; // Undefined behavior for invalid input, so we skip
        }

        let a = nums[0];
        let b = nums[1];

        // Termination condition: sum of the two integers is zero
        if a + b == 0 {
            break;
        }

        // Preconditions: first integer must be non-negative for square root
        if a < 0 {
            continue; // Skip invalid input as per precondition
        }

        // Calculate the result based on the specified mathematical expression
        let result = (a as f64).sqrt() + (b as f64); // Example expression: sqrt(a) + b

        // Output the result
        println!("{}", result);
    }
}
