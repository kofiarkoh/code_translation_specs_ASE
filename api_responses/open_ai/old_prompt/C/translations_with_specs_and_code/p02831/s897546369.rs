use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into two long integers
    let mut numbers = input.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    // Ensure we have exactly two numbers
    if numbers.len() != 2 {
        eprintln!("Expected two integers as input.");
        return;
    }

    let a = numbers[0];
    let b = numbers[1];

    // Check preconditions: a and b must be non-negative
    if a < 0 || b < 0 {
        eprintln!("Both a and b must be non-negative integers.");
        return;
    }

    // If b is 0, we cannot find a multiple of a that is divisible by b
    if b == 0 {
        eprintln!("Cannot find a multiple of a that is divisible by 0.");
        return;
    }

    // Find the smallest multiple of a that is also a multiple of b
    for i in 1..=b + 1 {
        if (a * i) % b == 0 {
            // Output the result
            println!("{}", a * i);
            break;
        }
    }
}
