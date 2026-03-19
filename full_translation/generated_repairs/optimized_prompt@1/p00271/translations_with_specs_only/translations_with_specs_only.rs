use std::io::{self, BufRead};

fn main() {
    // Create a vector to hold the input integers
    let mut numbers = Vec::new();
    
    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    // Read exactly 14 integers
    for line in handle.lines() {
        // Handle potential errors in reading lines and parsing integers
        match line {
            Ok(line) => {
                match line.trim().parse::<i32>() {
                    Ok(num) => numbers.push(num),
                    Err(_) => {
                        eprintln!("Invalid input: {}", line);
                        return;
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return;
            }
        }
    }
    
    // Ensure we have exactly 14 integers
    if numbers.len() != 14 {
        eprintln!("Expected exactly 14 integers, got {}", numbers.len());
        return;
    }
    
    // Compute and output the differences
    for i in (0..14).step_by(2) {
        // Subtract the second integer from the first in each pair
        let difference = numbers[i] - numbers[i + 1];
        println!("{}", difference);
    }
}
