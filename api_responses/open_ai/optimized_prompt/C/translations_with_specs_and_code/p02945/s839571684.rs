fn main() {
    use std::io::{self, Write};

    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());

    // Get the two integers
    let a = match nums.next() {
        Some(num) => num,
        None => return, // Handle case where input is invalid
    };
    let b = match nums.next() {
        Some(num) => num,
        None => return, // Handle case where input is invalid
    };

    // Calculate sum, difference, and product
    let sum = a + b;
    let difference = a - b;
    let product = a * b;

    // Initialize max to a value less than any possible computed result
    let mut max = i32::MIN;

    // Determine the maximum value
    if max < sum {
        max = sum;
    }
    if max < difference {
        max = difference;
    }
    if max < product {
        max = product;
    }

    // Print the maximum value
    println!("{}", max);
}
