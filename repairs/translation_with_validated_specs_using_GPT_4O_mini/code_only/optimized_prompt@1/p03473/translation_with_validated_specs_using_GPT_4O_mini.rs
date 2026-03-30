use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let m: i32 = input.trim().parse().expect("Failed to parse integer");
    
    // Calculate the result and print it
    println!("{}", 48 - m);
}
