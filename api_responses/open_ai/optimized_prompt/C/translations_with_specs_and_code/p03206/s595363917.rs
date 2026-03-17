use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    if let Ok(n) = input.trim().parse::<i32>() {
        // Match the integer to the corresponding output
        match n {
            25 => println!("Christmas"),
            24 => println!("Christmas Eve"),
            23 => println!("Christmas Eve Eve"),
            22 => println!("Christmas Eve Eve Eve"),
            _ => {} // No output for other integers
        }
    }
    // No error handling for non-integer inputs as per specification
}
