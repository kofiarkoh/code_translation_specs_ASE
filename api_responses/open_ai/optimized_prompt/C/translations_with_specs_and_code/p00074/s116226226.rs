use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        // Read a line of input
        if io::stdin().read_line(&mut input).is_err() {
            break; // Exit on read error
        }

        // Split the input into parts and parse them
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            break; // Terminate if not exactly three parts
        }

        // Parse hours, minutes, and seconds
        let h: i32 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => break, // Terminate on parse error
        };
        let m: i32 = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => break, // Terminate on parse error
        };
        let s: i32 = match parts[2].parse() {
            Ok(val) => val,
            Err(_) => break, // Terminate on parse error
        };

        // Check for termination condition
        if h == -1 || m == -1 || s == -1 {
            break; // Terminate on -1 input
        }

        // Calculate current seconds and remaining time until the next two-hour mark
        let cur_sec = h * 3600 + m * 60 + s;
        let mut remaining = 2 * 3600 - cur_sec;

        // Output the remaining time
        if remaining < 0 {
            remaining = 0; // Ensure remaining time is not negative
        }
        println!("{:02}:{:02}:{:02}",
                 remaining / 3600,
                 (remaining % 3600) / 60,
                 remaining % 60);

        // Calculate three times the remaining time
        remaining *= 3;

        // Output three times the remaining time
        println!("{:02}:{:02}:{:02}",
                 remaining / 3600,
                 (remaining % 3600) / 60,
                 remaining % 60);
    }
}
