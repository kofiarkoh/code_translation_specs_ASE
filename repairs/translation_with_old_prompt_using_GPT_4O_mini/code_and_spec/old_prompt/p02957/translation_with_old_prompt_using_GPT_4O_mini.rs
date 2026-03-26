use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into two long integers
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i64 = nums[0].parse().expect("Invalid input for a");
    let b: i64 = nums[1].parse().expect("Invalid input for b");

    // Check if the sum is odd
    if (a + b) % 2 != 0 {
        println!("IMPOSSIBLE");
        return;
    }

    // Print the average
    println!("{}", (a + b) / 2);
}
