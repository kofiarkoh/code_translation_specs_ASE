use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    let (l, r, d) = (nums[0], nums[1], nums[2]);

    // Preconditions check
    if l > r || d <= 0 {
        // According to the spec, we do not handle errors explicitly.
        // However, we assume inputs are valid as per the specification.
        return;
    }

    // Calculate the count of integers divisible by d in the range [L, R]
    let count = r / d - (l - 1) / d;

    // Output the result
    print!("{}", count);
    io::stdout().flush().expect("Failed to flush stdout");
}
