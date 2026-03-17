use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into two integers
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());
    
    let a = match nums.next() {
        Some(num) => num,
        None => return, // Handle case where no input is provided
    };
    
    let b = match nums.next() {
        Some(num) => num,
        None => return, // Handle case where only one input is provided
    };

    // Check for division by zero
    if b == 0 {
        eprintln!("Error: Division by zero is not allowed.");
        return; // Exit early if b is zero
    }

    // Calculate the ceiling of the division
    let c = if a % b == 0 {
        a / b
    } else {
        a / b + 1
    };

    // Print the result
    println!("{}", c);
}
