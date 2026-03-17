use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize a vector to store results
    let mut results = Vec::new();
    let mut current_number = String::new();

    // Process each character in the input
    for c in input.chars() {
        if c.is_digit(10) { // Check if the character is a digit
            current_number.push(c); // Build the current number
        } else if !current_number.is_empty() { // If we hit a non-digit and have a number
            // Parse the current number
            let num = current_number.parse::<u32>().unwrap(); // Safe due to precondition
            results.push(num.to_string()); // Store the result as a string
            current_number.clear(); // Clear for the next number
        }
    }

    // Handle the last number if the input ends with a digit
    if !current_number.is_empty() {
        let num = current_number.parse::<u32>().unwrap(); // Safe due to precondition
        results.push(num.to_string());
    }

    // Output results, ensuring each result is on a new line
    for result in results {
        println!("{}", result);
    }
}
