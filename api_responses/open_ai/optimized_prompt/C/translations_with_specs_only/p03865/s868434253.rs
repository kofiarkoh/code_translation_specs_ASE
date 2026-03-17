use std::io::{self, Read};

fn main() {
    // Read the input from standard input
    let mut input = String::new();
    // Read the input into the string
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check the length of the input to ensure it meets the precondition
    if input.len() < 1 || input.len() > 114514 {
        // Behavior is not defined for empty input or input exceeding max length
        return;
    }

    // Get the first and last characters of the input
    let first_char = input.chars().next().unwrap(); // Safe due to length check
    let last_char = input.chars().last().unwrap(); // Safe due to length check

    // Compare the first and last characters and output the result
    if first_char == last_char {
        println!("First");
    } else {
        println!("Second");
    }
}
