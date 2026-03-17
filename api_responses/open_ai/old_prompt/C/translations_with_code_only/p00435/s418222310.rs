use std::io::{self, Write};

fn main() {
    let mut a = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut a).expect("Failed to read line");
    
    // Remove the newline character if present
    if let Some('\n') = a.chars().next_back() {
        a.pop();
    }

    // Process each character in the string
    let mut result = String::new();
    for c in a.chars() {
        let transformed_char = if ('D'..='Z').contains(&c) {
            // If the character is between 'D' and 'Z', subtract 3
            (c as u8).wrapping_sub(3) as char
        } else {
            // Otherwise, add 23
            (c as u8).wrapping_add(23) as char
        };
        result.push(transformed_char);
    }

    // Print the result
    print!("{}\n", result);
}
