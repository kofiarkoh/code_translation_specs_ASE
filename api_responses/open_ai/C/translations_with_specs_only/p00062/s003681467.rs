use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Read each line and unwrap to handle the Result
        let input = line.unwrap();

        // Ensure the input is valid: up to 10 characters and numeric
        if input.len() > 10 || !input.chars().all(char::is_numeric) {
            // Undefined behavior for invalid input as per specification
            continue; // In practice, we would handle this, but spec says undefined
        }

        // Compute the single-digit result from the input string
        let result = reduce_to_single_digit(&input);
        
        // Print the result
        println!("{}", result);
    }
}

// Function to reduce the input string to a single-digit integer
fn reduce_to_single_digit(input: &str) -> u32 {
    let mut digits: Vec<u32> = input.chars()
        .map(|c| c.to_digit(10).unwrap()) // Safe because input is validated
        .collect();

    // Reduce the digits until we have a single digit
    while digits.len() > 1 {
        let mut new_digits = Vec::new();
        for i in 0..digits.len() - 1 {
            // Combine adjacent digits
            let combined = (digits[i] + digits[i + 1]) % 10; // Wrap around to single digit
            new_digits.push(combined);
        }
        digits = new_digits;
    }

    // Return the single remaining digit
    digits[0]
}
