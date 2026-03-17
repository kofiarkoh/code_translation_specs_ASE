use std::io::{self, Read};

fn main() {
    // Create a buffer to read input, with a maximum size of 114514 characters.
    let mut buffer = String::with_capacity(114514);
    
    // Read from standard input.
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        eprintln!("Error reading input: {}", e);
        return;
    }

    // Check the length of the input to ensure it meets the precondition.
    let length = buffer.len();
    if length == 0 {
        // According to the specification, we assume input is always valid and non-empty.
        // Thus, we do not handle this case explicitly.
        return;
    }

    // Get the first and last characters of the input.
    let first_char = buffer.chars().next().unwrap(); // Safe due to length check
    let last_char = buffer.chars().last().unwrap(); // Safe due to length check

    // Compare the first and last characters and output the result.
    if first_char == last_char {
        println!("First");
    } else {
        println!("Second");
    }
}
