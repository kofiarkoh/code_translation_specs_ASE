use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    let (L, R, d) = (nums[0], nums[1], nums[2]);

    // Preconditions check
    assert!(L <= R, "L must be less than or equal to R");
    assert!(d > 0, "d must be greater than 0");

    // Calculate the count of integers divisible by d in the range [L, R]
    let count = R / d - (L - 1) / d;

    // Output the result
    print!("{}", count);
    io::stdout().flush().expect("Failed to flush stdout");
}
