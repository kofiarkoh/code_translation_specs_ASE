use std::io::{self, Write};

fn main() {
    // Read two long integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Ensure we have exactly two parts
    if parts.len() != 2 {
        // If not, we cannot proceed; behavior is not defined for invalid input
        return;
    }

    // Parse the two integers
    let a: i64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => return, // Undefined behavior for invalid long integers
    };
    
    let b: i64 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => return, // Undefined behavior for invalid long integers
    };

    // Calculate the sum
    let sum = a + b;

    // Check if the sum is even
    if sum % 2 == 0 {
        // Calculate the average
        let average = sum / 2;
        println!("{}", average); // Output the average
    } else {
        // Output the impossibility message
        println!("Operation impossible"); // Indicate that the operation is impossible
    }
}
