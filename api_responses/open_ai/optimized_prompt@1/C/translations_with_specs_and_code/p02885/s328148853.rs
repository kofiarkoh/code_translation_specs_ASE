use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as integers
    let mut numbers = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());
    
    // Get the two integers, defaulting to 0 if not enough valid integers are provided
    let a = numbers.next().unwrap_or(0);
    let b = numbers.next().unwrap_or(0);
    
    // Calculate the result of the arithmetic operation
    let result = if a - 2 * b > 0 { a - 2 * b } else { 0 };
    
    // Print the result
    print!("{}", result);
}
