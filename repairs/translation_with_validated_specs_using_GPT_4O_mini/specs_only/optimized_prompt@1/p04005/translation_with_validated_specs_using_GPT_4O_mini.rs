use std::io;

fn main() {
    // Read three long integer values from user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly three numbers
    if nums.len() != 3 {
        panic!("Expected exactly three long integers");
    }

    // Check if any number is even
    if nums.iter().any(|&num| num % 2 == 0) {
        // If any number is even, output "0"
        println!("0");
    } else {
        // If all numbers are odd, find the maximum
        let max_value = *nums.iter().max().unwrap();
        // Calculate the product of the three numbers
        let product: i64 = nums.iter().product();
        // Replace the maximum value with 1 in the product calculation
        let adjusted_product = product / max_value; // Remove max_value from product
        let final_product = adjusted_product * 1; // Multiply by 1 instead of max_value
        println!("{}", final_product);
    }
}
