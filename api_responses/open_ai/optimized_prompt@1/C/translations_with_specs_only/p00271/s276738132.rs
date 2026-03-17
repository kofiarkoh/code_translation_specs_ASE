use std::io::{self, BufRead};

fn main() {
    // Create a vector to hold the input integers
    let mut numbers = Vec::new();
    
    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    // Read exactly 14 integers
    for line in handle.lines() {
        // Unwrap the line and parse it as an integer
        let line = line.unwrap();
        let num: i32 = line.trim().parse().unwrap(); // Assumes valid input as per specification
        numbers.push(num);
    }
    
    // Ensure we have exactly 14 integers
    assert_eq!(numbers.len(), 14, "Expected exactly 14 integers");
    
    // Compute and output the differences
    for i in (0..14).step_by(2) {
        // Subtract the second integer from the first in each pair
        let difference = numbers[i] - numbers[i + 1];
        println!("{}", difference);
    }
}
