use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a long long integer (i64 in Rust)
    let value: i64 = input.trim().parse().expect("Input is not a valid integer");

    // Compute the output based on the divisibility by 5
    let output = (value + 4) / 5; // Adjusted to round up for non-divisible cases

    // Output the result to standard output
    println!("{}", output);
}
