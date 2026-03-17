use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is shown before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a non-negative integer
    let n: u32 = input.trim().parse().expect("Input must be a non-negative integer");

    // Find the nearest palindrome
    let nearest_palindrome = find_nearest_palindrome(n);
    
    // Output the nearest palindrome
    println!("{}", nearest_palindrome);
}

// Function to check if a number is a palindrome
fn is_palindrome(num: u32) -> bool {
    let s = num.to_string();
    s == s.chars().rev().collect::<String>()
}

// Function to find the nearest palindrome to a given number
fn find_nearest_palindrome(n: u32) -> u32 {
    let mut lower = n;
    let mut upper = n;

    // Search for the nearest palindrome
    loop {
        if is_palindrome(lower) {
            return lower; // Return the lower palindrome if found
        }
        if is_palindrome(upper) {
            return upper; // Return the upper palindrome if found
        }
        // Decrement lower and increment upper to search for palindromes
        if lower > 0 {
            lower -= 1; // Ensure we do not go below 0
        }
        upper += 1; // Increment upper
    }
}
