use std::io::{self, Write};

const INPUT_NUM_MIN: u32 = 1; // Input range lower limit
const INPUT_NUM_MAX: u32 = 1_000_000_000; // Input range upper limit

fn main() {
    let mut input = String::new();

    // Read input from standard input
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Error reading input");
        std::process::exit(1); // Error reading input
    }

    // Parse input
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    if nums.len() != 2 || 
       nums[0].parse::<u32>().is_err() || 
       nums[1].parse::<u32>().is_err() {
        eprintln!("Invalid input");
        std::process::exit(1); // Invalid input
    }

    let u32_x: u32 = nums[0].parse().unwrap();
    let u32_y: u32 = nums[1].parse().unwrap();

    // Validate input range
    if (u32_x < INPUT_NUM_MIN || u32_x > INPUT_NUM_MAX) ||
       (u32_y < INPUT_NUM_MIN || u32_y > INPUT_NUM_MAX) {
        eprintln!("Input out of range");
        std::process::exit(1); // Input out of range
    }

    // Calculate GCD
    let u32_gcd = if u32_x > u32_y {
        calc_gcd_u32(u32_x, u32_y)
    } else {
        calc_gcd_u32(u32_y, u32_x)
    };

    // Print the GCD
    println!("{}", u32_gcd);
}

// Function to calculate GCD using the Euclidean algorithm
fn calc_gcd_u32(u32_large_num: u32, u32_small_num: u32) -> u32 {
    let mut large_num = u32_large_num;
    let mut small_num = u32_small_num;

    while small_num != 0 {
        let remainder = large_num % small_num;
        large_num = small_num;
        small_num = remainder;
    }

    large_num // Return the GCD
}
