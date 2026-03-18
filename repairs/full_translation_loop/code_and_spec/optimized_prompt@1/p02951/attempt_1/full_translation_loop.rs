use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the three integers from the input
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid input, expected three integers");
    
    // Ensure we have exactly three integers
    if nums.len() != 3 {
        panic!("Expected exactly three integers");
    }
    
    let (mut a, b, mut c) = (nums[0], nums[1], nums[2]);
    
    // Modify 'a' by subtracting 'b'
    a -= b;
    
    // Decrement 'a' and 'c' according to the specification
    while a > 0 {
        a -= 1;
        c -= 1;
        if c == 0 {
            break;
        }
    }
    
    // Output the final value of 'c'
    println!("{}", c);
}
