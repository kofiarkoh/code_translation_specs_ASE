use std::io::{self, BufRead};

fn main() {
    // Create a vector to hold the input integers
    let mut numbers = Vec::new();
    
    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    // Read exactly 14 integers
    for line in handle.lines() {
        // Unwrap the line and parse it into integers
        let line = line.unwrap();
        for num in line.split_whitespace() {
            // Parse each number and push it into the vector
            if let Ok(n) = num.parse::<i32>() {
                numbers.push(n);
            }
        }
    }
    
    // Ensure we have exactly 14 integers
    assert_eq!(numbers.len(), 14, "Expected exactly 14 integers");

    // Compute and print the differences for each pair
    for i in (0..14).step_by(2) {
        // Subtract the second integer from the first
        let difference = numbers[i] - numbers[i + 1];
        // Print the result
        println!("{}", difference);
    }
}
