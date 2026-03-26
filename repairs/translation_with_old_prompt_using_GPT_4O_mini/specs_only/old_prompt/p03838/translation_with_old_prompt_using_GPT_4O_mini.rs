use std::io::{self, Write};

fn main() {
    // Read input values for x and y
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Safely parse x and y as integers, handling cases where input may be invalid
    let x: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(value) => value,
        None => {
            eprintln!("Invalid input for x");
            return;
        }
    };
    
    let y: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(value) => value,
        None => {
            eprintln!("Invalid input for y");
            return;
        }
    };

    // Initialize result variable
    let result: i64;

    // Conditional logic based on the relationship between x and y
    if x > y {
        result = (x - y) as i64; // If x is greater than y, compute x - y
    } else if x < y {
        result = (y - x) as i64; // If x is less than y, compute y - x
    } else {
        result = 0; // If x equals y, the result is 0
    }

    // Print the result to standard output
    println!("{}", result);
}
``` 

// Note: The previous code was correct in logic but failed to handle the input correctly. The error message indicates that the input was not parsed correctly. The code now ensures that it reads the input correctly and handles any potential parsing errors gracefully.
