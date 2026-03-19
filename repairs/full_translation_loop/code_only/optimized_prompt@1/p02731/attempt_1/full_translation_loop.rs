use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let l: i32 = input.trim().parse().expect("Failed to parse input as integer");
    
    // Perform the division and cube calculation
    let y = l as f64 / 3.0; // Ensure we are using f64 for floating-point division
    let x = y * y * y; // Calculate y^3
    
    // Print the result with 12 decimal places
    // Using format! to ensure the output matches the C code's precision
    println!("{:.12}", x);
}
