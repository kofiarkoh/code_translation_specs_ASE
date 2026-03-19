use std::io::{self, Write};

fn main() {
    let mut n = 0;
    let mut count = 0;
    let mut string = String::new();

    // Read an integer from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    n = input.trim().parse().expect("Failed to parse integer");

    // Read a string from stdin
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    string = input.trim().to_string();

    // Ensure the string length does not exceed 51 characters
    if string.len() > 51 {
        panic!("Input string exceeds maximum length of 51 characters");
    }

    let chars: Vec<char> = string.chars().collect();

    // Iterate through the string, checking for "ABC"
    let mut i = 0;
    while i < n - 2 {
        if chars[i] == 'A' && chars[i + 1] == 'B' && chars[i + 2] == 'C' {
            count += 1;
            i += 2; // Skip the next two characters
        }
        i += 1; // Move to the next character
    }

    // Print the count
    print!("{}", count);
}
