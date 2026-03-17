use std::io::{self, Write};

fn main() {
    // Read the first input string
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).expect("Failed to read line");
    let first_input = first_input.trim(); // Remove any trailing newline characters

    // Read the second input string
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).expect("Failed to read line");
    let second_input = second_input.trim(); // Remove any trailing newline characters

    // Check preconditions: both strings must be non-empty and not exceed 100 characters
    if first_input.is_empty() || second_input.is_empty() || first_input.len() > 100 || second_input.len() > 100 {
        // According to the specification, we assume inputs are valid, so we do not handle errors here.
        return;
    }

    // Create the concatenated version of the first input string
    let concatenated = format!("{}{}", first_input, first_input);

    // Check if the second input string is a substring of the concatenated string
    if concatenated.contains(second_input) {
        println!("Yes");
    } else {
        println!("No");
    }
}
