use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the three integers from the input
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();
    
    // Ensure we have exactly three integers
    if nums.len() != 3 {
        panic!("Expected exactly three integers");
    }

    let (mut a, b, mut c) = (nums[0], nums[1], nums[2]);

    // Preconditions: c must be non-negative
    if c < 0 {
        panic!("The third integer must be non-negative");
    }

    // Modify a by subtracting b
    a -= b;

    // Invariants: Decrement a and c while a is positive
    while a > 0 {
        a -= 1;
        c -= 1;
        if c == 0 {
            break;
        }
    }

    // Output the final value of c
    println!("{}", c);
}
