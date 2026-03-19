use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Parse the input as a long long integer (i64 in Rust)
    let x: i64 = input.parse().expect("Input must be a valid integer");

    // Compute the output based on the specified conditions
    let output = if x % 11 == 0 {
        2 * (x / 11)
    } else if x % 11 > 6 {
        2 * (x / 11) + 2
    } else {
        2 * (x / 11) + 1
    };

    // Print the output
    println!("{}", output);
}
