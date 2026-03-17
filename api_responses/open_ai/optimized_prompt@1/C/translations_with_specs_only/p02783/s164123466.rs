use std::io::{self, Write};

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    // Ensure we have exactly two integers
    if nums.len() != 2 {
        panic!("Expected exactly two integers");
    }

    let (numerator, denominator) = (nums[0], nums[1]);

    // Check the precondition: the second integer must not be zero
    if denominator == 0 {
        // Undefined behavior as per specification
        panic!("Division by zero is undefined");
    }

    // Calculate the ceiling of the division
    let result = ceiling_div(numerator, denominator);

    // Print the result
    println!("{}", result);
}

// Function to calculate the ceiling of the division of two integers
fn ceiling_div(numerator: i32, denominator: i32) -> i32 {
    // If numerator is non-negative, we can use the following formula
    if numerator >= 0 {
        (numerator + denominator - 1) / denominator
    } else {
        // If numerator is negative, we need to handle it differently
        numerator / denominator
    }
}
