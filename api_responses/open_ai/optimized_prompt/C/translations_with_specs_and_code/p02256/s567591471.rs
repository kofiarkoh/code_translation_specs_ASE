use std::io::{self, Write};

const INPUT_NUM_MIN: u32 = 1; // Input range minimum
const INPUT_NUM_MAX: u32 = 1_000_000_000; // Input range maximum

fn main() {
    let mut input = String::new();
    let mut return_val = 0; // 0 for success, 1 for failure

    // Read input from stdin
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse input
    let nums: Vec<u32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.len() != 2 || nums[0] < INPUT_NUM_MIN || nums[0] > INPUT_NUM_MAX || 
       nums[1] < INPUT_NUM_MIN || nums[1] > INPUT_NUM_MAX {
        return_val = 1; // Set return value to 1 for failure
    } else {
        let gcd = calc_gcd(nums[0], nums[1]);
        println!("{}", gcd); // Output the GCD
    }

    std::process::exit(return_val);
}

// Function to calculate the GCD of two numbers
fn calc_gcd(mut large_num: u32, mut small_num: u32) -> u32 {
    while small_num != 0 {
        let remainder = large_num % small_num;
        large_num = small_num;
        small_num = remainder;
    }
    large_num // Return the GCD
}
