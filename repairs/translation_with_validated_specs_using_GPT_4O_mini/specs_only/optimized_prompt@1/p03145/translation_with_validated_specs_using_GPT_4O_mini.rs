use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly three integers
    if nums.len() < 2 {
        eprintln!("Error: Not enough valid integers provided.");
        return;
    }

    let a = nums[0];
    let b = nums[1];

    // Calculate half of the product of a and b
    let product = a as f64 * b as f64; // Convert to f64 for accurate division
    let half_product = product / 2.0;

    // Round to the nearest integer
    let output = half_product.round() as i32;

    // Print the output as an integer
    println!("{}", output);
}
