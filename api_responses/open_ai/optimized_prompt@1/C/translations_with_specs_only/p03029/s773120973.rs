use std::io::{self, Write};

fn main() {
    // Read two integer values from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure the output buffer is flushed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().expect("Expected first integer").parse().expect("Invalid integer");
    let p: i32 = iter.next().expect("Expected second integer").parse().expect("Invalid integer");
    
    // Calculate the result of the expression ((a * 3) + p) / 2
    let result = ((a * 3) + p) / 2;

    // Output the result
    println!("{}", result);
}
