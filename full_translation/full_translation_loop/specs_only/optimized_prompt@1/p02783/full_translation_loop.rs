use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as integers
    let mut parts = input.split_whitespace();
    let a: i32 = parts.next().expect("Expected first integer").parse().expect("Invalid integer");
    let b: i32 = parts.next().expect("Expected second integer").parse().expect("Invalid integer");
    
    // Preconditions check: b must not be zero
    if b == 0 {
        // Undefined behavior as per specification; we will panic here
        panic!("Division by zero is undefined");
    }
    
    // Calculate the ceiling of the division
    let result = (a + (b - 1)) / b; // This ensures we round up for positive a
    
    // Postcondition: output is greater than or equal to the division result
    // Invariants: output is non-negative if a is non-negative
    println!("{}", result);
}
