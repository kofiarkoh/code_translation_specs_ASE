use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Parse the input as a 64-bit signed integer
    let number: i64 = input.parse().unwrap(); // Assumes valid input as per specification

    // Compute the derived value based on divisibility by 11
    // The expected output seems to be the result of (number / 11) * 11 + (number % 11)
    let result = number - (number / 11) * 11; // This gives the same result as number % 11 but matches the expected output

    // Print the result followed by a newline
    println!("{}", result);
}
