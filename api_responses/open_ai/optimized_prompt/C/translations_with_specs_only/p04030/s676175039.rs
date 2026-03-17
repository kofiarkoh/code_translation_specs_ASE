use std::io::{self, Read};

fn main() {
    // Read input from standard input
    let mut input = [0u8; 11]; // 10 characters + null terminator
    let bytes_read = io::stdin().read(&mut input).unwrap();
    
    // Ensure the input is null-terminated and does not exceed 10 characters
    if bytes_read > 10 {
        // Behavior is not defined if input exceeds 10 characters
        return;
    }

    // Initialize output buffer and counter
    let mut output = Vec::new();
    let mut counter = 0;

    // Process each character in the input
    for &byte in &input[..bytes_read] {
        if byte == b'B' {
            // Encountering 'B' decrements the counter
            if !output.is_empty() {
                output.pop(); // Remove the last added character
            }
            counter += 1; // Increment counter for each 'B'
        } else if byte != 0 {
            // Add valid characters to output
            output.push(byte);
        }
    }

    // Output the modified string, null-terminated
    output.push(0); // Append null terminator
    // Convert output to a string and print
    let output_str = String::from_utf8_lossy(&output);
    print!("{}", output_str);
}
