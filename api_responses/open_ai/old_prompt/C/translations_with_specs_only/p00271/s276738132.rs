use std::io::{self, Write};

fn main() {
    // We will read pairs of integers and compute their difference for exactly 7 iterations.
    for _ in 0..7 {
        let mut input = String::new();
        
        // Read a line of input from the user
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Split the input into parts and parse them as integers
        let mut parts = input.split_whitespace();
        
        // Attempt to parse the first integer
        let a: i32 = parts.next().expect("Expected an integer").parse().expect("Invalid integer");
        // Attempt to parse the second integer
        let b: i32 = parts.next().expect("Expected an integer").parse().expect("Invalid integer");
        
        // Calculate the difference
        let difference = a - b;
        
        // Output the result followed by a newline
        println!("{}", difference);
    }
}
