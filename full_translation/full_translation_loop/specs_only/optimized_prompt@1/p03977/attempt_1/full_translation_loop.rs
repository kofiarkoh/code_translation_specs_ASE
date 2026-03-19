use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize a vector to store results
    let mut results = Vec::new();
    let mut current_number = String::new();

    // Process the input characters
    for ch in input.chars() {
        // Check if the character is a digit
        if ch.is_digit(10) {
            current_number.push(ch);
        } else {
            // If we have a complete number, process it
            if !current_number.is_empty() {
                // Convert the current number to an integer
                let num = current_number.parse::<u32>().unwrap(); // Safe due to precondition
                // Here we assume a simple operation, e.g., doubling the number
                let result = num * 2; // Example operation
                results.push(result.to_string());
                current_number.clear(); // Reset for the next number
            }
        }
    }

    // Process the last number if there is one
    if !current_number.is_empty() {
        let num = current_number.parse::<u32>().unwrap(); // Safe due to precondition
        let result = num * 2; // Example operation
        results.push(result.to_string());
    }

    // Output results, ensuring each result is on a new line
    for result in results {
        println!("{}", result);
    }
}
