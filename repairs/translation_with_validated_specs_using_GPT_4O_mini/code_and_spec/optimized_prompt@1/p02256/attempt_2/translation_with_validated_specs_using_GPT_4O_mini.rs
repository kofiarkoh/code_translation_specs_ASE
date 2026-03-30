use std::io::{self, Write};

const INPUT_NUM_MIN: u32 = 1; // Input range minimum
const INPUT_NUM_MAX: u32 = 1_000_000_000; // Input range maximum

fn main() -> Result<(), u8> {
    let mut input = String::new();

    // Read input
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    if io::stdin().read_line(&mut input).is_err() {
        return Err(1); // Return failure if input reading fails
    }

    // Parse input
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    if nums.len() != 2 {
        return Err(1); // Return failure if not exactly two numbers are provided
    }

    // Attempt to parse the numbers
    let (u32_x, u32_y) = match (nums[0].parse::<u32>(), nums[1].parse::<u32>()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return Err(1), // Return failure if parsing fails
    };

    // Validate input range
    if (u32_x < INPUT_NUM_MIN || u32_x > INPUT_NUM_MAX) || 
       (u32_y < INPUT_NUM_MIN || u32_y > INPUT_NUM_MAX) {
        return Err(1); // Return failure if inputs are out of range
    }

    // Calculate GCD
    let u32_gcd = if u32_x > u32_y {
        calc_gcd(u32_x, u32_y)
    } else {
        calc_gcd(u32_y, u32_x)
    };

    // Print GCD
    println!("{}", u32_gcd);
    Ok(()) // Return success
}

fn calc_gcd(u32_large_num: u32, u32_small_num: u32) -> u32 {
    let mut large_num = u32_large_num;
    let mut small_num = u32_small_num;

    while small_num != 0 {
        let remainder = large_num % small_num;
        large_num = small_num;
        small_num = remainder;
    }

    large_num // Return the GCD
}
