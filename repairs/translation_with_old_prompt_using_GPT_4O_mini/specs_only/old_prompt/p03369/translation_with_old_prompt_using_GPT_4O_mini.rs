use std::io::{self, Write};

fn main() {
    // Initialize the score starting at 700
    let mut score = 700;

    // Read a string of exactly 3 characters from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Count occurrences of 'o' in the input string
    // The specification guarantees that the input will be exactly 3 characters long
    for c in input.chars() {
        if c == 'o' {
            score += 100; // Increase score by 100 for each 'o' found
        }
    }

    // Print the calculated score
    println!("{}", score);
}
