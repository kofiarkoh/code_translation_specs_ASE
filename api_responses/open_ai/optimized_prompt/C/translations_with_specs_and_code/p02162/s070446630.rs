use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    if io::stdin().read_line(&mut input).is_err() {
        return; // Terminate if reading fails
    }

    // Split the input into parts and parse them
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    // Check if we have exactly four integers
    if parts.len() != 4 {
        return; // Terminate without output if not enough inputs
    }

    // Parse the integers
    let t1: i32 = parts[0].parse().unwrap_or(0);
    let t2: i32 = parts[1].parse().unwrap_or(0);
    let r1: i32 = parts[2].parse().unwrap_or(0);
    let r2: i32 = parts[3].parse().unwrap_or(0);

    // Determine the winner based on the specifications
    let result = if r1 < 0 || r2 < 0 {
        if t1 < t2 {
            "Alice"
        } else if t1 > t2 {
            "Bob"
        } else {
            "Draw"
        }
    } else {
        if r1 > r2 {
            "Alice"
        } else if r1 < r2 {
            "Bob"
        } else {
            "Draw"
        }
    };

    // Output the result
    println!("{}", result);
}
