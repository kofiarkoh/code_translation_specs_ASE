use std::io::{self, Read};

fn main() {
    // Create a buffer to hold up to 114514 characters
    let mut buffer = [0u8; 114514]; // Using u8 to read raw bytes

    // Read from standard input
    let bytes_read = io::stdin().read(&mut buffer).unwrap_or(0);

    // Check if we have at least one character
    if bytes_read == 0 {
        // If no input is provided, we do not output anything
        return;
    }

    // Convert the bytes to characters
    let input = std::str::from_utf8(&buffer[..bytes_read]).unwrap();

    // Get the first and last characters
    let first_char = input.chars().next().unwrap();
    let last_char = input.chars().last().unwrap();

    // Compare the first and last characters and output the result
    if first_char == last_char {
        println!("First");
    } else {
        println!("Second");
    }
}
