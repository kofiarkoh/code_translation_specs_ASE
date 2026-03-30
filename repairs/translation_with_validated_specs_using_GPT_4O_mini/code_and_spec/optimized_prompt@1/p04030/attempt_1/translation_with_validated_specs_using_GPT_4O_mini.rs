use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input string, ensuring it does not exceed 10 characters
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Trim the input to remove any trailing newline characters
    let input = input.trim();
    
    // Ensure the input is null-terminated and does not exceed 10 characters
    if input.len() > 10 {
        panic!("Input exceeds maximum length of 10 characters");
    }

    let mut output = Vec::new();
    let mut j = 0;

    for &c in input.as_bytes() {
        if c == b'B' {
            if j > 0 {
                j -= 1; // Remove the last character added to output
                output.pop(); // Safely remove the last character from the Vec
            }
        } else {
            output.push(c); // Add character to output
            j += 1;
            if c == b'\0' { // Check for null terminator
                break;
            }
        }
    }

    // Convert output Vec<u8> back to a String
    let output_string = String::from_utf8_lossy(&output).to_string();
    
    // Print the output string
    print!("{}", output_string);
}
