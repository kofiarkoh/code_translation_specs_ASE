use std::io::{self, BufRead};

fn main() {
    // Read a single line of input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap_or(Ok(String::new())).unwrap();

    // Initialize a vector to hold valid words
    let mut valid_words = Vec::new();

    // Split the input by spaces, commas, and periods
    for word in input.split(|c: char| c == ' ' || c == ',' || c == '.') {
        let trimmed_word = word.trim(); // Trim whitespace

        // Check the length of the trimmed word
        if trimmed_word.len() >= 3 && trimmed_word.len() <= 6 {
            valid_words.push(trimmed_word.to_string());
        }
    }

    // Join valid words with a space and print the result
    // If there are no valid words, this will result in an empty string
    let output = valid_words.join(" ");
    println!("{}", output); // Output the result followed by a newline
}
