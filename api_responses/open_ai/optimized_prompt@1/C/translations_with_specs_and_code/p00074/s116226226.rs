use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        // Read input from stdin
        if io::stdin().read_line(&mut input).is_err() {
            continue; // Handle read error by continuing to wait for valid input
        }

        // Parse the input into three integers
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            continue; // Invalid input, continue waiting for valid input
        }

        let h: i32 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => continue, // Handle parse error by continuing to wait for valid input
        };
        let m: i32 = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => continue, // Handle parse error by continuing to wait for valid input
        };
        let s: i32 = match parts[2].parse() {
            Ok(val) => val,
            Err(_) => continue, // Handle parse error by continuing to wait for valid input
        };

        // Check for termination condition
        if h == -1 || m == -1 || s == -1 {
            break; // Exit the loop if termination condition is met
        }

        // Validate time components
        if h < 0 || h >= 24 || m < 0 || m >= 60 || s < 0 || s >= 60 {
            continue; // Invalid time components, continue waiting for valid input
        }

        // Calculate current seconds and remaining time until the next two-hour mark
        let cur_sec = h * 3600 + m * 60 + s;
        let mut remaining = 2 * 3600 - cur_sec;

        // Output the remaining time until the next two-hour mark
        if remaining < 0 {
            remaining = 0; // Ensure remaining time is not negative
        }
        println!("{:02}:{:02}:{:02}",
                 remaining / 3600,
                 (remaining % 3600) / 60,
                 remaining % 60);

        // Calculate and output three times the remaining time
        remaining *= 3;
        println!("{:02}:{:02}:{:02}",
                 remaining / 3600,
                 (remaining % 3600) / 60,
                 remaining % 60);
    }
}
