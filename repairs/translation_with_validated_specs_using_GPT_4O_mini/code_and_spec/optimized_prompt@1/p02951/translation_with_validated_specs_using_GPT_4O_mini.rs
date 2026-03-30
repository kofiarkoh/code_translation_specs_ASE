use std::io::{self, Write};

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the three integers from input
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Ensure we have exactly three integers
    if nums.len() != 3 {
        panic!("Input must consist of exactly three integers.");
    }

    let (mut a, b, mut c) = (nums[0], nums[1], nums[2]);

    // Modify 'a' based on the difference of the first two integers
    a -= b;

    // Ensure 'a' is non-negative as per the specification
    if a < 0 {
        a = 0;
    }

    // Decrement 'c' while 'a' is positive and 'c' is greater than zero
    while a > 0 && c > 0 {
        a -= 1;
        c -= 1;
    }

    // Output the final value of 'c'
    println!("{}", c);
}
