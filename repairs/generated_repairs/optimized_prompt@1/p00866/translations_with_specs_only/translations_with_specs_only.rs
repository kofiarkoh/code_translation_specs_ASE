use std::io::{self, BufRead};

fn main() {
    // Read the number of clock configurations
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Parse the number of configurations
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Invalid number of configurations"),
        _ => return, // Exit if input is not valid
    };

    // Ensure n is positive
    if n == 0 {
        return; // No valid output for zero configurations
    }

    let mut times = Vec::new();

    // Read each configuration
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() != 3 {
                return; // Invalid input format
            }

            // Parse hour, minute, second
            let hour: i32 = parts[0].parse().unwrap_or(-1);
            let minute: i32 = parts[1].parse().unwrap_or(-1);
            let second: i32 = parts[2].parse().unwrap_or(-1);

            // Validate the clock hand positions
            if hour < 0 || hour > 59 || minute < 0 || minute > 59 || second < 0 || second > 59 {
                return; // Invalid clock hand positions
            }

            // Store the valid time as a tuple
            times.push((hour, minute, second));
        }
    }

    // Determine the minimum and maximum times
    if times.is_empty() {
        return; // No valid times to process
    }

    let mut min_time = times[0];
    let mut max_time = times[0];

    for &time in &times {
        if time < min_time {
            min_time = time;
        }
        if time > max_time {
            max_time = time;
        }
    }

    // Output the results in the specified format
    println!("{:02}:{:02}:{:02}", min_time.0 / 10, min_time.1 / 10, min_time.2 / 10);
    println!("{:02}:{:02}:{:02}", max_time.0 / 10, max_time.1 / 10, max_time.2 / 10);
}
