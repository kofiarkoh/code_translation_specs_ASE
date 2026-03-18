use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next();

    // Check if we have exactly one line of input
    if let Some(Ok(line)) = input {
        // Split the line into parts and parse them as integers
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Ensure we have exactly four integer values
        if parts.len() == 4 {
            // Parse the time and result values
            let time_a: i32 = parts[0].parse().unwrap();
            let time_b: i32 = parts[1].parse().unwrap();
            let result_a: i32 = parts[2].parse().unwrap_or(0); // Default to 0 if parsing fails
            let result_b: i32 = parts[3].parse().unwrap_or(0); // Default to 0 if parsing fails

            // Determine the winner based on the specified rules
            if time_a < time_b {
                println!("Alice");
            } else if time_a > time_b {
                println!("Bob");
            } else {
                // If times are equal, compare results
                if result_a < 0 || result_b < 0 {
                    // If results are negative, ignore them as per the specification
                    println!("Draw");
                } else if result_a > result_b {
                    println!("Alice");
                } else if result_a < result_b {
                    println!("Bob");
                } else {
                    println!("Draw");
                }
            }
        }
        // If we do not have exactly four integers, we do not output anything
    }
}
