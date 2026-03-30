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
    let mut result = String::with_capacity(a.len());
    for c in a.chars() {
        let processed_char = if ('D'..='Z').contains(&c) {
            // Subtract 3 from characters in the range 'D' to 'Z'
            (c as u8).wrapping_sub(3) as char
        } else {
            // Add 23 to all other characters
            (c as u8).wrapping_add(23) as char
        };
        result.push(processed_char);
    }

    // Print the result
    println!("{}", result);
}
