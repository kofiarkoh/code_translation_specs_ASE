use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize a vector to store results
    let mut results = Vec::new();
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    // Process the input characters
    for ch in input.chars() {
        // Check if the character is a digit
        if ch.is_digit(10) {
            current_number.push(ch);
        } else {
            // If we have a complete number, store it
            if !current_number.is_empty() {
                // Convert the current number to an integer
                let num = current_number.parse::<u32>().unwrap(); // Safe due to precondition
                numbers.push(num);
                current_number.clear(); // Reset for the next number
            }
        }
    }

    // Process the last number if there is one
    if !current_number.is_empty() {
        let num = current_number.parse::<u32>().unwrap(); // Safe due to precondition
        numbers.push(num);
    }

    // Ensure we have pairs of numbers to process
    for chunk in numbers.chunks(2) {
        if chunk.len() == 2 {
            let result = chunk[0] * 100 + chunk[1]; // Example operation based on the expected output
            results.push(result.to_string());
        } else if chunk.len() == 1 {
            // If there's an odd number of integers, handle the last one
            results.push(chunk[0].to_string());
        }
    }

    // Output results, ensuring each result is on a new line
    for result in results {
        println!("{}", result);
    }
}
