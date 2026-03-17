use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize a vector to store results
    let mut results = Vec::new();
    let mut output_size = 0;

    // Split the input into valid integers based on non-numeric characters
    let mut numbers: Vec<u32> = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.is_empty() {
            // Parse the current number and reset
            if let Ok(num) = current_number.parse::<u32>() {
                numbers.push(num);
            }
            current_number.clear();
        }
    }
    // Handle the last number if there is one
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<u32>() {
            numbers.push(num);
        }
    }

    // Process pairs of integers
    for chunk in numbers.chunks(2) {
        if chunk.len() == 2 {
            let result = chunk[0] + chunk[1]; // Example operation: sum
            let result_str = format!("{}\n", result);
            output_size += result_str.len();
            results.push(result_str);
        }
    }

    // Ensure output size does not exceed 20,000 characters
    if output_size <= 20_000 {
        // Write results to standard output
        for result in results {
            print!("{}", result);
        }
    }
}
