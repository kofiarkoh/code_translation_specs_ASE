use std::io;

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the two integers from the input
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Ensure we have exactly two integers
    if nums.len() != 2 {
        panic!("Expected exactly two integers as input");
    }

    let a = nums[0];
    let b = nums[1];

    // Compute the sum, difference, and product
    let sum = a + b;
    let difference = a - b;
    let product = a * b;

    // Initialize max to a value less than any possible computed result
    let mut max = i32::MIN;

    // Update max based on the computed values
    if max < sum {
        max = sum;
    }
    if max < difference {
        max = difference;
    }
    if max < product {
        max = product;
    }

    // Output the maximum value
    println!("{}", max);
}
