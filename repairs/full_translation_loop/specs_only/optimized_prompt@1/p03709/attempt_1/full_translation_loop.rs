use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of pairs
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize variables to hold the computed result
    let mut computed_result: i64 = 0; // Using i64 to represent a long integer

    // Process each pair of integers
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Assuming the input is always valid as per the specification
        let a = nums[0];
        let b = nums[1];

        // Perform calculations based on the input pairs
        // Here we assume some arbitrary computation for demonstration
        computed_result += (a * b) as i64; // Example computation
    }

    // Determine the output based on the computed result
    let output = if computed_result > 0 { "Yes" } else { "No" };

    // Print the results
    println!("{}", output);
    println!("{}", computed_result);
}
