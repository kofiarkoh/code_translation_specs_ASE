use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Ensure the input does not exceed 99 characters
    let input = if input.len() > 99 {
        &input[..99] // Truncate to 99 characters if necessary
    } else {
        &input
    };

    let mut frequency = [0; 256]; // Frequency array for ASCII characters
    let mut unique_count = 0; // Count of unique characters
    let mut consecutive_count = 0; // Count of consecutive characters

    // Count the frequency of each character in the input string
    for &byte in input.as_bytes() {
        frequency[byte as usize] += 1;
    }

    // Process the frequency array
    let mut remaining = input.len(); // Remaining characters to process
    while remaining > 0 {
        for i in 0..256 {
            if frequency[i] > 0 {
                unique_count += 1; // Found a unique character
                remaining -= 1; // Decrease remaining count
                consecutive_count += 1; // Increase consecutive count
                frequency[i] -= 1; // Decrease frequency of the character
            } else {
                // If there are no more of this character
                if consecutive_count > 3 {
                    unique_count -= consecutive_count - 3; // Apply penalty
                }
                consecutive_count = 0; // Reset consecutive count
            }
        }
    }

    // Output the result
    println!("{}", unique_count);
}
