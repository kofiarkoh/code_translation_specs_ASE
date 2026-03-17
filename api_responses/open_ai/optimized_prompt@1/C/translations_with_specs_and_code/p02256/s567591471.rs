use std::io::{self, Write};

const INPUT_NUM_MIN: u32 = 1; // Input range lower limit
const INPUT_NUM_MAX: u32 = 1_000_000_000; // Input range upper limit

fn main() {
    let mut input = String::new();
    let mut return_val = 0; // 0 for success, 1 for failure

    // Read input from stdin
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse input
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    if nums.len() != 2 {
        return_val = 1; // Invalid input
    } else {
        let u32_x: u32 = match nums[0].parse() {
            Ok(num) if num >= INPUT_NUM_MIN && num <= INPUT_NUM_MAX => num,
            _ => {
                return_val = 1; // Out of range
                0 // Placeholder, won't be used
            }
        };

        let u32_y: u32 = match nums[1].parse() {
            Ok(num) if num >= INPUT_NUM_MIN && num <= INPUT_NUM_MAX => num,
            _ => {
                return_val = 1; // Out of range
                0 // Placeholder, won't be used
            }
        };

        if return_val == 0 {
            // Calculate GCD
            let u32_gcd = if u32_x > u32_y {
                calc_gcd(u32_x, u32_y)
            } else {
                calc_gcd(u32_y, u32_x)
            };
            println!("{}", u32_gcd); // Output GCD
        }
    }

    std::process::exit(return_val); // Exit with the appropriate status
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
