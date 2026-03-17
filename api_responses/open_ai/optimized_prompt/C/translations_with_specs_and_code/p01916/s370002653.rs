use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Ensure the input length does not exceed 1009 characters
    assert!(input.len() <= 1009, "Input exceeds maximum length of 1009 characters.");

    // Array to track distinct lowercase letters
    let mut distinct_chars = [false; 26]; // For 'a' to 'z'
    let mut count = 0;

    // Iterate over each character in the input string
    for c in input.chars() {
        if c.is_ascii_lowercase() {
            let index = (c as usize) - ('a' as usize);
            if !distinct_chars[index] {
                distinct_chars[index] = true;
                count += 1; // Increment count for each distinct character
            }
        }
    }

    // Output half the count of distinct characters
    println!("{}", count / 2);
}
