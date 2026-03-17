use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    loop {
        input.clear();
        // Read a line of input
        if io::stdin().read_line(&mut input).is_err() {
            break; // Exit on read error
        }

        // Parse the input into hours, minutes, and seconds
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            break; // Exit if we don't have exactly 3 parts
        }

        let h: i32 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => break, // Exit on parse error
        };
        let m: i32 = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => break, // Exit on parse error
        };
        let s: i32 = match parts[2].parse() {
            Ok(val) => val,
            Err(_) => break, // Exit on parse error
        };

        // Check for termination condition
        if h == -1 && m == -1 && s == -1 {
            break;
        }

        // Calculate current seconds
        let cur_sec = h * 3600 + m * 60 + s;
        let remaining = 2 * 3600 - cur_sec;

        // Print the first time
        print_time(remaining);

        // Calculate and print the second time
        let remaining = remaining * 3;
        print_time(remaining);
    }
}

// Function to print time in HH:MM:SS format
fn print_time(total_seconds: i32) {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    // Print formatted time
    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}
