use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Ensure the input length does not exceed 100 characters
    let input = if input.len() > 100 {
        &input[..100] // Truncate to 100 characters if necessary
    } else {
        &input
    };

    let mut frequency = [0; 256]; // Frequency array for ASCII characters
    let mut unique_count = 0; // Count of unique characters
    let mut current_streak = 0; // Current streak of consecutive characters

    // Count the frequency of each character in the input string
    for &byte in input.as_bytes() {
        frequency[byte as usize] += 1;
    }

    // Process the frequency array to calculate the result
    let mut remaining = input.len(); // Remaining characters to process
    while remaining > 0 {
        for i in 0..256 {
            if frequency[i] > 0 {
                unique_count += 1; // Found a unique character
                remaining -= 1; // Decrease remaining count
                current_streak += 1; // Increase current streak
                frequency[i] -= 1; // Decrease frequency of the character
            } else {
                if current_streak > 3 {
                    unique_count -= current_streak - 3; // Adjust unique count based on streak
                }
                current_streak = 0; // Reset current streak
            }
        }
    }

    // Output the result
    println!("{}", unique_count);
}
