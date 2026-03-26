use std::io::{self, Write};

fn main() {
    // Create a buffer to hold the input
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().expect("Expected first integer").parse().expect("Invalid integer");
    let p: i32 = iter.next().expect("Expected second integer").parse().expect("Invalid integer");
    
    // Calculate the result based on the specification
    let result = (a * 3 + p) / 2;
    
    // Print the result followed by a newline
    println!("{}", result);
}
