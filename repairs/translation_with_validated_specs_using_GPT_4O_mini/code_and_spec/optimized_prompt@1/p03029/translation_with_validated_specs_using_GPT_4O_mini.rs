use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into two integers
    let mut numbers = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());
    
    // Extract the two integers a and p
    let a = numbers.next().expect("Failed to read first integer");
    let p = numbers.next().expect("Failed to read second integer");
    
    // Compute the result based on the specified formula
    let result = (a * 3 + p) / 2;
    
    // Print the result
    println!("{}", result);
}
