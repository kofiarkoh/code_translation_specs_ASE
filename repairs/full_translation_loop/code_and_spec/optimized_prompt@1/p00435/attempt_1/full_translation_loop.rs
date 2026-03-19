use std::io::{self, Write};

fn main() {
    // Create a buffer to hold the input string, with a maximum length of 999 characters.
    let mut input = String::new();
    
    // Read a single line of input from standard input.
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Trim the input to remove any trailing newline characters.
    let input = input.trim();
    
    // Ensure the input does not exceed the maximum length of 999 characters.
    if input.len() > 999 {
        // Behavior is not defined for inputs exceeding the maximum length.
        return;
    }
    
    // Create a mutable vector to hold the transformed characters.
    let mut output: Vec<char> = Vec::with_capacity(input.len());
    
    // Transform each character according to the specified rules.
    for c in input.chars() {
        let transformed_char = if ('D'..='Z').contains(&c) {
            // Shift uppercase letters backward by three positions.
            (c as u8).wrapping_sub(3) as char
        } else {
            // Shift all other characters forward by twenty-three positions.
            (c as u8).wrapping_add(23) as char
        };
        output.push(transformed_char);
    }
    
    // Convert the output vector back to a string.
    let output_string: String = output.iter().collect();
    
    // Print the transformed string followed by a newline.
    println!("{}", output_string);
}
