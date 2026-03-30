use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize a vector to store the results
    let mut results = Vec::new();
    let mut chars = input.chars().peekable();
    
    // Function to extract the next non-negative integer from the input
    fn next_integer(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<u32> {
        let mut number_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                number_str.push(c);
                chars.next(); // consume the character
            } else {
                break; // stop on non-digit character
            }
        }
        if number_str.is_empty() {
            None // No valid number found
        } else {
            number_str.parse::<u32>().ok() // Parse and return the number
        }
    }

    // Process pairs of integers
    while let (Some(first), Some(second)) = (next_integer(&mut chars), next_integer(&mut chars)) {
        let result = if first % 2 == 1 {
            // N is odd
            127 * (first - 1) + second
        } else {
            // N is even
            127 * first - second
        };
        results.push(result.to_string());
    }

    // Output results, each followed by a newline
    for result in results {
        println!("{}", result);
    }
}
