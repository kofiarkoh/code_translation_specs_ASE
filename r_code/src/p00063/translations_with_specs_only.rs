use std::io::{self, BufRead};

const MAX_STRING_LENGTH: usize = 256; // Define a maximum length for input strings

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut palindrome_count = 0;

    for line in reader.lines() {
        match line {
            Ok(input) => {
                // Ensure the input does not exceed the maximum length
                if input.len() > MAX_STRING_LENGTH {
                    continue; // Skip inputs that are too long
                }
                if is_palindrome(&input) {
                    palindrome_count += 1; // Increment count if palindrome
                }
            }
            Err(_) => {
                // Handle potential read errors (not specified in the task)
                continue; // Skip on error
            }
        }
    }

    println!("{}", palindrome_count); // Output the count of palindromes
}
