use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let l: i32 = input.trim().parse().expect("Failed to parse input as integer");
    
    // Perform the division, ensuring to use floating-point division
    let y = l as f64 / 3.0;
    
    // Calculate x as y cubed
    let x = y * y * y;
    
    // Print the result with 12 decimal places
    // Using format! to ensure the output matches the C code's precision
    println!("{:.12}", x);
}
