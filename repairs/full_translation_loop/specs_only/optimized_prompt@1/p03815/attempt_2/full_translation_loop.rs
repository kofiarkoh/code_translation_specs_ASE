use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as an integer
    let value: i64 = input.trim().parse().expect("Input is not a valid integer");

    // Compute the output based on the integer division of value by 11
    // The output should be the ceiling of value / 11
    let output = (value + 10) / 11; // This effectively computes the ceiling of value / 11

    // Output the result
    println!("{}", output);
}
