use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    if nums.len() != 3 {
        eprintln!("Expected three positive integers.");
        return;
    }

    let (a, b, c) = (nums[0], nums[1], nums[2]);

    // Preconditions: a, b, and c must be greater than 0
    if a <= 0 || b <= 0 || c <= 0 {
        eprintln!("All inputs must be positive integers.");
        return;
    }

    let mut sum = 0;
    let mut l = -1; // Initialize l to -1 to indicate not found

    // Determine the range to iterate over
    let max = a.max(b);
    
    for i in (1..=max).rev() {
        if a % i == 0 && b % i == 0 {
            sum += 1;
            if sum == c {
                l = i;
                break; // Found the c-th common divisor
            }
        }
    }

    // Output the result
    if l != -1 {
        println!("{}", l);
    } else {
        // If c-th common divisor does not exist, behavior is undefined
        // According to the spec, we may not output anything or handle it gracefully
        eprintln!("The {}-th common divisor does not exist.", c);
    }
}
