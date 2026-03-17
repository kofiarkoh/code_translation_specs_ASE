fn main() {
    use std::io::{self, Write};

    // Create a buffer to read the input string
    let mut input = String::new();

    // Read the input string from stdin
    io::stdout().flush().unwrap(); // Ensure the output buffer is flushed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check that the input length is exactly 3 characters
    if input.len() != 3 {
        eprintln!("Input must consist of exactly three characters.");
        return;
    }

    // Count occurrences of 'o'
    let count_o = input.chars().filter(|&c| c == 'o').count();

    // Calculate the score
    let score = 700 + (100 * count_o);

    // Output the score
    println!("{}", score);
}
