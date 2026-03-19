use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i64> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.len() != 2 {
        eprintln!("Expected two integers as input.");
        return;
    }

    let n = nums[0];
    let m = nums[1];

    // Preconditions: m must be greater than or equal to 1
    if m < 1 {
        eprintln!("Precondition failed: m must be >= 1");
        return;
    }

    let sum: i64 = if m == 1 {
        0
    } else if (m / 2 - n) < 0 {
        m / 2
    } else if (m / 2 - n) < 2 {
        n
    } else {
        n + (m / 2 - n) / 2
    };

    // Postcondition: Output value is guaranteed to be non-negative
    println!("{}", sum);
}
