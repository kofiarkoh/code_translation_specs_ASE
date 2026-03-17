use std::io::{self, Write};

fn main() {
    // Initialize the score starting at 700
    let mut score = 700;

    // Read a string of exactly 3 characters from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is shown before input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check if the input length is exactly 3 characters
    if input.len() == 3 {
        // Count occurrences of 'o' in the input string
        for c in input.chars() {
            if c == 'o' {
                score += 100; // Increase score for each 'o' found
            }
        }
    } // If the input is not exactly 3 characters, the behavior is undefined

    // Print the final score
    println!("{}", score);
}
