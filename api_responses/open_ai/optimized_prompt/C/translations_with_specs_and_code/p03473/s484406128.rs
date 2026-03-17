use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Attempt to parse the input as an integer
    match input.trim().parse::<i32>() {
        Ok(m) => {
            // Calculate the difference from 48 and print the result
            println!("{}", 48 - m);
        },
        Err(_) => {
            // If parsing fails, behavior is not defined as per the spec
            // No output or error handling is specified in the spec
        }
    }
}
