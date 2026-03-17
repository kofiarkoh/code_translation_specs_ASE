use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the three integers from the input
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect::<Vec<i32>>();

    let (mut a, mut b, mut c) = (nums[0], nums[1], nums[2]);
    
    // Perform the subtraction
    a -= b;

    // Loop until a is no longer greater than 0
    while a > 0 {
        a -= 1; // Decrement a
        c -= 1; // Decrement c
        if c == 0 {
            break; // Exit if c reaches 0
        }
    }

    // Print the result
    println!("{}", c);
}
