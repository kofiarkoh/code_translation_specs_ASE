use std::io::{self, Write};

fn main() {
    // Create a buffer to hold the input string, with a maximum length of 999 characters.
    let mut input = String::new();
    
    // Read a line of input from the user.
    // The input is expected to be a single string without spaces.
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters.
    let input = input.trim();
    
    // Create a mutable String to hold the transformed characters.
    let mut transformed = String::with_capacity(input.len());
    
    // Process each character in the input string according to the specified rules.
    for c in input.chars() {
        let transformed_char = if ('D'..='Z').contains(&c) {
            // Shift uppercase letters backward by 3.
            (c as u8).wrapping_sub(3) as char
        } else {
            // Shift other characters forward by 23.
            (c as u8).wrapping_add(23) as char
        };
        transformed.push(transformed_char);
    }
    
    // Print the transformed string.
    println!("{}", transformed);
}
