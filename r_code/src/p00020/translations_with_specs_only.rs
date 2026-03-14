use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Ensure the input does not exceed 80 characters
    // The specification states that behavior is undefined if it exceeds this length
    if input.len() > 80 {
        // Undefined behavior as per specification; we will just return
        return;
    }

    // Convert all lowercase alphabetic characters to uppercase
    let output: String = input.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect();

    // Output the modified string
    // Append a null terminator to the output as per the specification
    // In Rust, we can simulate this by printing a null character
    print!("{}{}", output, '\0');
    io::stdout().flush().expect("Failed to flush stdout");
}
