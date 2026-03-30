use std::io::{self, Read};

fn main() {
    // Define the maximum buffer size as specified
    const MAX_BUFFER_SIZE: usize = 114514;
    let mut buf = [0; MAX_BUFFER_SIZE];

    // Read from standard input
    let n = io::stdin().read(&mut buf).expect("Failed to read from stdin");

    // Ensure we have at least one character to compare
    if n == 0 {
        // According to the spec, behavior is undefined for zero-length input
        return;
    }

    // Compare the first and last character
    // n is the number of bytes read, so the last character is at n - 1
    let first_char = buf[0] as char;
    let last_char = buf[n - 1] as char;

    // Output "First" if they are equal, otherwise "Second"
    if first_char == last_char {
        println!("First");
    } else {
        println!("Second");
    }
}
