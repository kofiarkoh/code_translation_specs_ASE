use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Ensure the input length does not exceed 99 characters
    let input = &input[..input.len().min(99)];
    
    let mut char_count = HashMap::new();
    let mut unique_count = 0;
    let mut consecutive_count = 0;
    let mut last_char = None;

    for c in input.chars() {
        // Count occurrences of each character
        *char_count.entry(c).or_insert(0) += 1;
    }

    // Process the character counts
    for (&c, &count) in &char_count {
        unique_count += 1; // Count unique characters
        if let Some(last) = last_char {
            if last == c {
                consecutive_count += count;
            } else {
                if consecutive_count > 3 {
                    unique_count -= consecutive_count - 3; // Adjust for excess
                }
                consecutive_count = count; // Reset for new character
            }
        } else {
            consecutive_count = count; // First character
        }
        last_char = Some(c);
    }

    // Final adjustment for the last character
    if consecutive_count > 3 {
        unique_count -= consecutive_count - 3;
    }

    // Ensure the unique count is non-negative
    unique_count = unique_count.max(0);

    // Print the result
    println!("{}", unique_count);
}
