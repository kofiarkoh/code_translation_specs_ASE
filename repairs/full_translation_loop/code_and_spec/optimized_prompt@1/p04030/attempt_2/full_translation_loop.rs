use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin, ensuring it does not exceed 10 characters
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Trim the input to remove any trailing newline characters
    let input = input.trim();
    
    // Ensure the input is null-terminated and does not exceed 10 characters
    if input.len() > 10 {
        // Behavior is not defined for inputs longer than 10 characters
        return;
    }

    let mut output = Vec::new(); // Use a Vec to manage output dynamically
    let mut removal_count = 0; // Counter for characters to remove

    for c in input.chars() {
        if c == 'B' {
            removal_count += 1; // Increment removal count for each 'B'
        } else {
            if removal_count > 0 {
                removal_count -= 1; // Decrement removal count if there are characters to remove
            } else {
                output.push(c); // Add character to output if not removed
            }
        }
    }

    // Convert output Vec to String and print
    let output_string: String = output.into_iter().collect();
    print!("{}", output_string);
}
