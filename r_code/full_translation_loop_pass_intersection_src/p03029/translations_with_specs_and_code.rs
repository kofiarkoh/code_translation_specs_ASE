use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into two integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().expect("Expected an integer").parse().expect("Invalid integer");
    let p: i32 = iter.next().expect("Expected an integer").parse().expect("Invalid integer");

    // Calculate the result based on the specification
    let result = ((a * 3) + p) / 2;

    // Output the result to standard output
    println!("{}", result);
}
