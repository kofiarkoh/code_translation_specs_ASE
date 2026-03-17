use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Ensure the input length does not exceed 1009 characters
    assert!(input.len() <= 1009, "Input exceeds maximum length of 1009 characters.");

    // Array to track distinct lowercase alphabetic characters
    let mut distinct_chars = [false; 26];

    // Count distinct characters
    for c in input.chars() {
        if c.is_ascii_lowercase() {
            distinct_chars[(c as usize) - ('a' as usize)] = true;
        }
    }

    // Calculate the number of distinct characters
    let distinct_count = distinct_chars.iter().filter(|&&x| x).count();

    // Output the number of distinct characters
    println!("{}", distinct_count);
}
