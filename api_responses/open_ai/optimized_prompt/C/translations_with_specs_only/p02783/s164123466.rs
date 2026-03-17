use std::io::{self, Write};

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Ensure we have exactly two integers
    if nums.len() != 2 {
        panic!("Expected exactly two integers");
    }

    let a = nums[0];
    let b = nums[1];

    // Check the precondition: the second integer must not be zero
    if b == 0 {
        // Undefined behavior as per specification
        panic!("Division by zero is undefined");
    }

    // Calculate the ceiling of the division
    let result = ceiling_div(a, b);

    // Output the result followed by a newline
    println!("{}", result);
}

// Function to calculate the ceiling of the division of two integers
fn ceiling_div(numerator: i32, denominator: i32) -> i32 {
    // The ceiling of the division can be calculated using the formula:
    // (numerator + denominator - 1) / denominator
    // This works for positive and negative integers.
    if (numerator > 0 && denominator > 0) || (numerator < 0 && denominator < 0) {
        (numerator + denominator - 1) / denominator
    } else {
        numerator / denominator
    }
}
