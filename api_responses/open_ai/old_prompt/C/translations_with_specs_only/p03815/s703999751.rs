use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Parse the input as a 64-bit signed integer (i64)
    let number: i64 = input.parse().expect("Invalid input format");

    // Compute the derived value based on divisibility by 11
    let result = number % 11;

    // Print the result followed by a newline
    println!("{}", result);
}
