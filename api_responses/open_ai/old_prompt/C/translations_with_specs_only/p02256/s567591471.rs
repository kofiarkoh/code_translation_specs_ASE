use std::io::{self, Write};

fn gcd(mut a: u32, mut b: u32) -> u32 {
    // Euclidean algorithm to compute GCD
    while b != 0 {
        let temp = b;
        b = a % b; // Remainder
        a = temp;  // Update a
    }
    a // Return GCD
}

fn main() -> i32 {
    let mut input = String::new();
    
    // Read input from standard input
    if io::stdin().read_line(&mut input).is_err() {
        return 1; // Return 1 on read error
    }

    // Split input into two parts and parse them
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return 1; // Return 1 if input does not contain exactly two numbers
    }

    // Parse the two unsigned integers
    let u32_x: u32 = match parts[0].parse() {
        Ok(num) if num >= 1 && num <= 1_000_000_000 => num,
        _ => return 1, // Return 1 on invalid input
    };

    let u32_y: u32 = match parts[1].parse() {
        Ok(num) if num >= 1 && num <= 1_000_000_000 => num,
        _ => return 1, // Return 1 on invalid input
    };

    // Compute the GCD
    let result = gcd(u32_x, u32_y);

    // Output the result
    println!("{}", result);
    
    0 // Return 0 for successful execution
}
