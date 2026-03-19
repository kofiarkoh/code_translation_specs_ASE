use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as an integer
    let value: i64 = input.trim().parse().expect("Input is not a valid integer");

    // Compute the output based on the integer division of value by 11
    // The output should be the ceiling of value / 11
    let output = if value >= 0 {
        (value + 10) / 11 // This computes the ceiling of value / 11 for non-negative values
    } else {
        value / 11 // For negative values, we just perform integer division
    };

    // Output the result
    println!("{}", output);
}
