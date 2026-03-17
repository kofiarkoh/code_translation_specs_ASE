use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let p: i32 = iter.next().unwrap().parse().expect("Failed to parse p");
    
    // Calculate the result as per the C code
    let result = (a * 3 + p) / 2;
    
    // Print the result, preserving the format
    println!("{}", result);
}
