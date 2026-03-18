use std::io::{self, Write};

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as integers
    let mut parts = input.split_whitespace();
    let a: i32 = parts.next().expect("Expected first integer").parse().expect("Invalid integer");
    let p: i32 = parts.next().expect("Expected second integer").parse().expect("Invalid integer");
    
    // Calculate the result of the expression ((a * 3) + p) / 2
    let result = ((a * 3) + p) / 2; // Integer division is used to ensure the result is an integer
    
    // Output the result
    println!("{}", result);
}
