use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read three long integers from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly three numbers
    assert_eq!(nums.len(), 3);

    // Check if any number is even
    if nums[0] % 2 == 0 || nums[1] % 2 == 0 || nums[2] % 2 == 0 {
        // Output "0" if any number is even
        println!("0");
    } else {
        // Find the maximum number and its index
        let mut max_index = 0;
        for i in 1..3 {
            if nums[i] > nums[max_index] {
                max_index = i;
            }
        }

        // Set the maximum number to 1
        let mut modified_nums = nums.clone();
        modified_nums[max_index] = 1;

        // Calculate the product of the three numbers
        let product = modified_nums[0] * modified_nums[1] * modified_nums[2];
        println!("{}", product);
    }
}
