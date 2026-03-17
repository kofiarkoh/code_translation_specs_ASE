use std::io::{self, Read};

fn main() {
    // Create a buffer with a maximum length of 114514 characters
    let mut buf = [0u8; 114514];
    
    // Read from standard input
    let n = io::stdin().read(&mut buf).expect("Failed to read from stdin");
    
    // Ensure we have at least one character
    if n == 0 {
        // Behavior is not defined for empty input, but we will not handle it explicitly as per the spec
        return;
    }

    // Convert the first and last characters to char for comparison
    let first_char = buf[0] as char;
    let last_char = buf[n - 1] as char;

    // Compare the first and last characters and print the appropriate output
    if first_char == last_char {
        println!("First");
    } else {
        println!("Second");
    }
}
