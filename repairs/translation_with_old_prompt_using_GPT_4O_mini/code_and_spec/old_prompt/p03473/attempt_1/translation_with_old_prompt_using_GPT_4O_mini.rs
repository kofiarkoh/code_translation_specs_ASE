use std::io::{self, Write};

fn main() {
    // Create a variable to hold the user input
    let mut input = String::new();

    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as an integer
    let m: i32 = input.trim().parse().expect("Invalid input, expected an integer");

    // Calculate the result
    let result = 48 - m;

    // Print the result followed by a newline
    println!("{}", result);
}
