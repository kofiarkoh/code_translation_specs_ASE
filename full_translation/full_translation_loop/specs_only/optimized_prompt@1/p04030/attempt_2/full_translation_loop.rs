use std::io::{self, Read};

fn main() {
    // Read input from standard input
    let mut input = [0u8; 11]; // 10 characters + null terminator
    let bytes_read = io::stdin().read(&mut input).unwrap();
    
    // Ensure the input is null-terminated
    if bytes_read == 0 || input[bytes_read - 1] != 0 {
        return; // Undefined behavior for invalid input
    }

    // Process the input string
    let mut output = Vec::new();
    let mut remove_count = 0;

    for &byte in &input[..bytes_read] {
        if byte == b'B' {
            remove_count += 1; // Increment removal counter
        } else if remove_count > 0 {
            remove_count -= 1; // Decrement removal counter for non-'B' characters
        } else {
            output.push(byte); // Add character to output if not removed
        }
    }

    // Append null terminator to output
    output.push(0);

    // Print the modified string to standard output
    // Convert Vec<u8> to a string slice for printing
    // We need to ensure we only print valid UTF-8 characters
    if let Ok(output_str) = std::str::from_utf8(&output) {
        // We need to print only the characters before the null terminator
        if let Some(pos) = output_str.find('\0') {
            print!("{}", &output_str[..pos]);
        } else {
            print!("{}", output_str);
        }
    }
}
