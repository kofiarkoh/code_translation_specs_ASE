use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a long long integer (i64 in Rust)
    let x: i64 = input.trim().parse().expect("Invalid input");

    // Compute the output based on the specified conditions
    let output = if x % 11 == 0 {
        2 * (x / 11)
    } else if x % 11 > 6 {
        2 * (x / 11) + 2
    } else {
        2 * (x / 11) + 1
    };

    // Print the output to standard output
    println!("{}", output);
}
