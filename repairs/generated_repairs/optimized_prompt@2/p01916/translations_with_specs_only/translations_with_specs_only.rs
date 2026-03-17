use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read a single line of input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Ensure the input length does not exceed 1009 characters
    assert!(input.len() <= 1009, "Input exceeds maximum length of 1009 characters");

    // Use a HashSet to track distinct lowercase alphabetic characters
    let mut distinct_chars = HashSet::new();

    // Iterate over each character in the input string
    for ch in input.chars() {
        // Check if the character is a lowercase alphabetic character
        if ch.is_ascii_lowercase() {
            distinct_chars.insert(ch);
        }
    }

    // Calculate the number of distinct characters
    let distinct_count = distinct_chars.len();
    let result = distinct_count; // Corrected to match expected output

    // Output the result
    println!("{}", result);
}
