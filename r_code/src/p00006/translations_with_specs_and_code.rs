use std::io::{self, Write};

fn main() {
    // Create a buffer to read the input string
    let mut input = String::new();
    
    // Read a line from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let input = input.trim();
    
    // Check the length constraint
    if input.len() > 20 {
        // Behavior is not defined if the input exceeds 20 characters
        // In a real application, we might handle this case more gracefully
        return;
    }
    
    // Reverse the string
    let reversed: String = input.chars().rev().collect();
    
    // Print the reversed string
    print!("{}", reversed);
}
