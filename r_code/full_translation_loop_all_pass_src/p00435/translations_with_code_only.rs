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
        let new_char = if ('D'..='Z').contains(&c) {
            // Shift character down by 3
            (c as u8).wrapping_sub(3) as char
        } else {
            // Shift character up by 23
            (c as u8).wrapping_add(23) as char
        };
        result.push(new_char);
    }

    // Print the result
    println!("{}", result);
}
