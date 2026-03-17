use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Ensure the input does not exceed 99 characters
    let input = if input.len() > 99 {
        &input[..99] // Truncate to 99 characters to prevent overflow
    } else {
        &input
    };

    // Initialize character frequency array for ASCII characters
    let mut frequency = [0; 256];
    let mut unique_count = 0;
    let mut consecutive_count = 1;

    // Iterate through the input string
    for (i, c) in input.chars().enumerate() {
        // Count unique characters
        if frequency[c as usize] == 0 {
            unique_count += 1;
        }
        frequency[c as usize] += 1;

        // Handle consecutive characters
        if i > 0 && c == input.chars().nth(i - 1).unwrap() {
            consecutive_count += 1;
        } else {
            // Apply penalty if consecutive characters exceed 2
            if consecutive_count > 2 {
                unique_count -= consecutive_count - 2; // Adjust unique count
            }
            consecutive_count = 1; // Reset for the new character
        }
    }

    // Final adjustment for the last sequence of characters
    if consecutive_count > 2 {
        unique_count -= consecutive_count - 2; // Adjust unique count
    }

    // Output the final score
    println!("{}", unique_count);
}
