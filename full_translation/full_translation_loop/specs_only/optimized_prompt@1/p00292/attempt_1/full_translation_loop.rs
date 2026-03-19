use std::io::{self, BufRead};

fn main() {
    // Read the number of pairs from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of pairs
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Expected a positive integer");

    // Ensure the number of pairs is positive
    assert!(n > 0, "The number of pairs must be positive");

    // Process each pair
    for _ in 0..n {
        // Read the next line containing the pair of integers
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Expected an integer"))
            .collect();

        // Ensure we have exactly two integers
        assert!(nums.len() == 2, "Each pair must consist of exactly two integers");

        let first = nums[0];
        let second = nums[1];

        // Calculate the output based on the first and second integers
        // The specific calculation is not defined in the specification,
        // so we will use a placeholder operation (e.g., first + second).
        let output = first + second; // Replace with the actual formula if specified

        // Print the output for the current pair
        println!("{}", output);
    }
}
