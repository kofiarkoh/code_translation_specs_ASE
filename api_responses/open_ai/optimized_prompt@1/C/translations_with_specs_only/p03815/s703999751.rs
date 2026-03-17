use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a long long integer (i64 in Rust)
    let value: i64 = input.trim().parse().expect("Input is not a valid integer");

    // Compute the output based on the divisibility by 11
    let output = if value % 11 == 0 {
        // If divisible by 11, return the integer division of value by 11
        value / 11
    } else {
        // If not divisible by 11, return the integer division of value by 11 plus 1
        value / 11 + 1
    };

    // Ensure the output is non-negative
    // Since value is a long long, the output will always be >= 0 based on the specification
    // Output the result to standard output
    println!("{}", output);
}
