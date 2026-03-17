use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Create a vector to store results
    let mut results = Vec::new();

    // Iterate over each line of input
    for line in reader.lines() {
        // Unwrap the line to handle the Result type
        let line = line.unwrap();
        
        // Split the line into tokens and collect them into a vector
        let tokens: Vec<&str> = line.split_whitespace().collect();
        
        // Ensure we have exactly two tokens (pairs of integers)
        if tokens.len() == 2 {
            // Parse the integers from the tokens
            let a: i32 = tokens[0].parse().unwrap();
            let b: i32 = tokens[1].parse().unwrap();

            // Compute the output based on the parity of the integers
            let result = if (a % 2 == 0) && (b % 2 == 0) {
                a + b // both even
            } else if (a % 2 != 0) && (b % 2 != 0) {
                a * b // both odd
            } else {
                a - b // one even, one odd
            };

            // Store the computed result
            results.push(result);
        }
    }

    // Output all computed results
    for result in results {
        println!("{}", result);
    }
}
