use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok()) // Only keep valid integers
        .collect();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        panic!("Expected exactly three integers");
    }

    let (mut a, b, c) = (numbers[0], numbers[1], numbers[2]);

    // Preconditions: c must be non-negative
    if c < 0 {
        panic!("The third integer must be non-negative");
    }

    // Step 1: Subtract the first integer from the second
    let mut result = b - a;

    // Step 2: Decrement the result by the minimum of the result and the third integer
    let decrement = result.min(c);
    result -= decrement;

    // Postcondition: Output the final value of the modified integer
    println!("{}", result);
}
