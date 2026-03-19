use std::io::{self, Write};

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure the output buffer is flushed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Ensure exactly two integers are provided
    if nums.len() != 2 {
        eprintln!("Error: Exactly two integers are required.");
        return;
    }

    let a = nums[0];
    let b = nums[1];

    // Calculate sum, difference, and product
    let sum = a + b;
    let difference = a - b;
    let product = a * b;

    // Determine the maximum value among sum, difference, and product
    let max_value = sum.max(difference).max(product);

    // Print the maximum value
    println!("{}", max_value);
}
