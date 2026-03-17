use std::io::{self, Read};

fn main() {
    // Create a buffer to hold up to 114514 characters
    let mut buffer = [0u8; 114514]; // Using u8 to read raw bytes from input

    // Read from standard input
    let bytes_read = io::stdin().read(&mut buffer).unwrap_or(0);

    // Check if we have at least one character
    if bytes_read == 0 {
        // If no input is provided, we do not output anything
        return;
    }

    // Convert the bytes read into a string slice
    let input = std::str::from_utf8(&buffer[..bytes_read]).unwrap();

    // Check the first and last characters
    let first_char = input.chars().next().unwrap(); // Safe due to previous check
    let last_char = input.chars().last().unwrap(); // Safe due to previous check

    // Output "First" if they are the same, otherwise "Second"
    if first_char == last_char {
        println!("First");
    } else {
        println!("Second");
    }
}
