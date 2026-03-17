use std::io::{self, Write};

fn main() {
    // Read two integer values from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and attempt to parse them as integers
    let mut numbers = input.split_whitespace();
    let first: i32 = match numbers.next().and_then(|s| s.parse().ok()) {
        Some(num) => num,
        None => return, // If parsing fails, exit the program (undefined behavior)
    };
    
    let second: i32 = match numbers.next().and_then(|s| s.parse().ok()) {
        Some(num) => num,
        None => return, // If parsing fails, exit the program (undefined behavior)
    };

    // Perform the arithmetic operation (subtraction instead of addition)
    let result = first % second; // Change to modulus operation

    // Print the result
    println!("{}", result);
}
