use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the number of datasets
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_datasets: usize = input.trim().parse().expect("Invalid number");

    for _ in 0..num_datasets {
        let mut dataset = String::new();
        
        // Read two strings representing non-negative integers
        io::stdin().read_line(&mut dataset).expect("Failed to read line");
        let parts: Vec<&str> = dataset.trim().split_whitespace().collect();
        
        if parts.len() != 2 {
            continue; // Skip if not exactly two parts
        }

        let (num_str1, num_str2) = (parts[0], parts[1]);

        // Check for overflow based on string length
        if num_str1.len() > 80 || num_str2.len() > 80 {
            println!("overflow");
            continue; // Skip to the next dataset
        }

        // Convert strings to integers
        let num1: u128 = match num_str1.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("overflow");
                continue; // Skip to the next dataset
            }
        };

        let num2: u128 = match num_str2.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("overflow");
                continue; // Skip to the next dataset
            }
        };

        // Compute the sum
        let sum = num1.checked_add(num2);
        
        // Check for overflow in the sum
        match sum {
            Some(result) => println!("{}", result),
            None => println!("overflow"), // Output overflow if sum exceeds u128
        }
    }
}
