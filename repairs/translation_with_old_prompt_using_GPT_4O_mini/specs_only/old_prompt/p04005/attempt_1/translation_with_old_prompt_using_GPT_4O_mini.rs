use std::io::{self, Write};

fn main() {
    // Read three long integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly three numbers
    assert_eq!(nums.len(), 3);

    // Check if any number is even
    if nums.iter().any(|&x| x % 2 == 0) {
        // If any number is even, output "0"
        println!("0");
    } else {
        // If all numbers are odd, find the maximum
        let mut max_index = 0;
        for i in 1..3 {
            if nums[i] > nums[max_index] {
                max_index = i;
            }
        }

        // Set the maximum number to 1 (this is a logical operation, not modifying the original array)
        let product = nums.iter().product::<i64>() / nums[max_index] * 1;

        // Output the product
        println!("{}", product);
    }

    // Return 0 upon successful execution
    std::process::exit(0);
}
