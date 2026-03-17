use std::io::{self, Write};

fn main() {
    // Read the number of clock configurations
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the number of configurations
    let n: i32 = input.trim().parse().expect("Invalid input");
    
    // Check for the precondition: number of configurations must be positive
    if n <= 0 {
        return; // Terminate without processing further
    }

    let mut min_time = i32::MAX; // Initialize to maximum possible value
    let mut max_time = i32::MIN; // Initialize to minimum possible value

    // Process each configuration
    for _ in 0..n {
        let mut config_input = String::new();
        io::stdin().read_line(&mut config_input).expect("Failed to read line");
        
        // Parse the hour, minute, and second
        let parts: Vec<i32> = config_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        
        // Ensure we have exactly three parts
        if parts.len() != 3 {
            continue; // Skip invalid configurations
        }

        let (h, m, s) = (parts[0], parts[1], parts[2]);

        // Check if the positions are valid (0 to 59)
        if h < 0 || h > 59 || m < 0 || m > 59 || s < 0 || s > 59 {
            continue; // Skip invalid configurations
        }

        // Calculate the total seconds from the clock hand positions
        let total_seconds = h * 3600 + m * 60 + s;

        // Update min and max times
        if total_seconds < min_time {
            min_time = total_seconds;
        }
        if total_seconds > max_time {
            max_time = total_seconds;
        }
    }

    // Format the output for minimum and maximum times
    let min_h = (min_time / 3600) % 24; // Convert seconds to hours
    let min_m = (min_time / 60) % 60;   // Convert seconds to minutes
    let min_s = min_time % 60;          // Remaining seconds

    let max_h = (max_time / 3600) % 24; // Convert seconds to hours
    let max_m = (max_time / 60) % 60;   // Convert seconds to minutes
    let max_s = max_time % 60;          // Remaining seconds

    // Output the results in HH:MM:SS format
    println!("{:02}:{:02}:{:02} {:02}:{:02}:{:02}", min_h, min_m, min_s, max_h, max_m, max_s);
}
