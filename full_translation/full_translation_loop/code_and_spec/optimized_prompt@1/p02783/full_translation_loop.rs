use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the two integers from the input
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());
    
    let a = match nums.next() {
        Some(num) => num,
        None => return, // Handle case where no input is provided
    };
    
    let b = match nums.next() {
        Some(num) => num,
        None => return, // Handle case where only one input is provided
    };

    // Preconditions: b must not be zero
    if b == 0 {
        // Undefined behavior as per specification; we could handle this gracefully
        eprintln!("Error: Division by zero is undefined.");
        return;
    }

    // Calculate the ceiling of the division
    let mut c = a / b;
    if a % b != 0 {
        c += 1; // Increment if there's a remainder
    }

    // Output the result
    println!("{}", c);
}
