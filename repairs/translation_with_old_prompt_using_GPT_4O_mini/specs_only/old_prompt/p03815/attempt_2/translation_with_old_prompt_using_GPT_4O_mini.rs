use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Parse the input as a 64-bit signed integer
    let number: i64 = input.parse().unwrap(); // Assumes valid input as per specification

    // Compute the derived value based on divisibility by 11
    // The specification implies we need to return the positive remainder
    let result = (number % 11 + 11) % 11; // Ensures the result is always non-negative

    // Print the result followed by a newline
    println!("{}", result);
}
