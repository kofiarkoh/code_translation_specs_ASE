use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        // Read a line of input
        if io::stdin().read_line(&mut input).is_err() {
            continue; // Handle read error by continuing to the next iteration
        }

        // Parse the input into three integers
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            continue; // Skip invalid input
        }

        let h: i32 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => continue, // Skip invalid input
        };
        let m: i32 = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => continue, // Skip invalid input
        };
        let s: i32 = match parts[2].parse() {
            Ok(val) => val,
            Err(_) => continue, // Skip invalid input
        };

        // Check for termination condition
        if h == -1 && m == -1 && s == -1 {
            break; // Exit the loop if all inputs are -1
        }

        // Validate time components
        if h < 0 || m < 0 || s < 0 {
            continue; // Skip invalid input
        }

        // Calculate current seconds and remaining time
        let cur_sec = h * 3600 + m * 60 + s;
        let remaining = 2 * 3600 - cur_sec;

        // Output the remaining time in HH:MM:SS format
        if remaining >= 0 {
            println!("{:02}:{:02}:{:02}", 
                remaining / 3600, 
                (remaining % 3600) / 60, 
                remaining % 60);
            
            // Calculate three times the remaining time
            let remaining_tripled = remaining * 3;
            println!("{:02}:{:02}:{:02}", 
                remaining_tripled / 3600, 
                (remaining_tripled % 3600) / 60, 
                remaining_tripled % 60);
        }
    }
}
