use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin, ensuring it does not exceed 10 characters
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let input = input.trim();
    
    // Ensure the input is null-terminated and does not exceed 10 characters
    if input.len() > 10 {
        // Behavior is not defined for inputs longer than 10 characters
        return;
    }

    let mut output = Vec::new();
    let mut j = 0;

    for &c in input.as_bytes() {
        if c == b'B' {
            if j > 0 {
                j -= 1; // Decrement the counter if possible
            }
        } else {
            output.push(c); // Add character to output
            j += 1; // Increment the counter
        }
    }

    // Convert output to a string and append a null terminator
    let output_str = String::from_utf8(output).expect("Invalid UTF-8 sequence");
    print!("{}", output_str); // Print the output string
    // Note: Rust strings are not null-terminated, but the specification does not require it in output.
}
