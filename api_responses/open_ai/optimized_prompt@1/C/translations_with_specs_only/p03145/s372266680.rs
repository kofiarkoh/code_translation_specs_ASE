use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly three inputs
    if inputs.len() < 2 {
        eprintln!("Error: Not enough input values.");
        return;
    }

    // Extract the first two integers for calculation
    let a = inputs[0];
    let b = inputs[1];

    // Calculate the product and then half of it, rounding to the nearest whole number
    let product = a as i64 * b as i64; // Use i64 to prevent overflow
    let half_product = (product + 1) / 2; // Round to nearest whole number

    // Ensure the output is non-negative if inputs are non-negative
    if a >= 0 && b >= 0 {
        println!("{}", half_product);
    } else {
        // If either input is negative, we still output the result
        println!("{}", half_product);
    }
}
