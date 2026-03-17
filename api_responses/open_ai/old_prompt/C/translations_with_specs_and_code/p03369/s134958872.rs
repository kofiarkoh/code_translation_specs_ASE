use std::io::{self, Write};

fn main() {
    // Create a buffer to hold the input string
    let mut input = String::new();

    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Ensure the input is exactly 3 characters long
    if input.len() != 3 {
        // Undefined behavior as per the specification; we will not handle this case explicitly.
        // In a real-world scenario, we might want to handle this gracefully.
        return;
    }

    // Initialize the score
    let mut score = 700;

    // Count occurrences of 'o'
    for c in input.chars() {
        if c == 'o' {
            score += 100;
        }
    }

    // Print the final score
    println!("{}", score);
}
