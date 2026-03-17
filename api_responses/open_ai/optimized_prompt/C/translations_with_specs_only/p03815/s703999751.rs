use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as an integer
    let value: i64 = input.trim().parse().expect("Invalid input"); // Behavior is undefined for invalid input

    // Compute the output based on the input value
    let output = value / 11; // Integer division by 11

    // Adjust output based on the remainder when divided by 11
    // Since the output must be >= 0, we ensure that we do not produce negative values
    let remainder = value % 11;
    let adjusted_output = if remainder >= 0 { output } else { output + 1 }; // This handles negative values correctly

    // Output the result
    println!("{}", adjusted_output);
}
