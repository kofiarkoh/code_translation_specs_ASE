use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly three valid integers
    if numbers.len() != 3 {
        panic!("Expected exactly three valid integers");
    }

    let (mut a, b, c) = (numbers[0], numbers[1], numbers[2]);

    // Preconditions: c must be non-negative
    if c < 0 {
        panic!("The third integer must be non-negative");
    }

    // Step 1: Modify the first integer based on the second
    a -= b; // Reduce a by b

    // Step 2: Calculate the result
    let result = a - std::cmp::min(a, c); // Decrement by the minimum of a and c

    // Output the final value
    println!("{}", result);
}
