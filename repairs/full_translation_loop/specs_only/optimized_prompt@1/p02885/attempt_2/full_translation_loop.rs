use std::io;

fn main() {
    // Read a single line of input containing two integers
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into two parts and attempt to parse them as integers
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return; // Undefined behavior if input does not contain exactly two integers
    }

    let num1: i32 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return, // Undefined behavior if input is not a valid integer
    };

    let num2: i32 = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => return, // Undefined behavior if input is not a valid integer
    };

    // Perform the arithmetic operation (num1 - num2) and ensure non-negativity
    let result = num1.checked_sub(num2).unwrap_or(0); // Use checked_sub to handle underflow

    // Output the result, ensuring it is non-negative
    println!("{}", result.max(0)); // Ensure output is non-negative
}
