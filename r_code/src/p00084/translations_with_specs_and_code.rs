use std::io::{self, BufRead};

fn main() {
    // Read a line of input from standard input
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap_or(Ok(String::new())).unwrap();

    // Split the line into words based on spaces, commas, and periods
    let delimiters = [' ', ',', '.'];
    let words: Vec<&str> = line.split(&delimiters[..]).collect();

    // Filter words based on length criteria (3 to 6 characters)
    let filtered_words: Vec<&str> = words
        .iter()
        .filter(|&&word| word.len() >= 3 && word.len() <= 6)
        .collect();

    // Join the filtered words with a space and print the result
    let output = filtered_words.join(" ");
    println!("{}", output);
}
