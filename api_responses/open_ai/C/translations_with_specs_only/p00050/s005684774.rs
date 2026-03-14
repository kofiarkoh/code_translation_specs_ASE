use std::io::{self, Read};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", e);
        return;
    }

    // Trim the input to remove any trailing newline characters
    let input = input.trim_end();

    // Process the input string
    let output = replace_fruits(input);

    // Print the modified string
    println!("{}", output);
}

fn replace_fruits(input: &str) -> String {
    // Create a mutable String to hold the output
    let mut output = String::new();
    let mut i = 0;

    while i < input.len() {
        // Check for "apple" or "peach" at the current position
        if input[i..].starts_with("apple") {
            // Replace "apple" with "peach"
            output.push_str("peach");
            i += 5; // Move past "apple"
        } else if input[i..].starts_with("peach") {
            // Replace "peach" with "apple"
            output.push_str("apple");
            i += 5; // Move past "peach"
        } else {
            // Copy the current character to the output
            output.push(input.chars().nth(i).unwrap());
            i += 1; // Move to the next character
        }
    }

    output
}
