use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of clock configurations
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Invalid number of configurations"),
        _ => return, // Exit if input is not valid
    };

    let mut times = Vec::new();

    // Read each clock configuration
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() != 3 {
                return; // Exit if the line does not contain exactly three integers
            }

            let hour: i32 = parts[0].parse().expect("Invalid hour");
            let minute: i32 = parts[1].parse().expect("Invalid minute");
            let second: i32 = parts[2].parse().expect("Invalid second");

            // Validate the clock hand positions
            if hour < 0 || hour > 59 || minute < 0 || minute > 59 || second < 0 || second > 59 {
                return; // Exit if any value is out of bounds
            }

            // Convert time to total seconds for easier comparison
            let total_seconds = hour * 3600 + minute * 60 + second;
            times.push(total_seconds);
        }
    }

    // Check if we have valid times
    if times.is_empty() {
        return; // No valid configurations to process
    }

    // Find the minimum and maximum times
    let min_time = *times.iter().min().unwrap();
    let max_time = *times.iter().max().unwrap();

    // Calculate the minimum and maximum times in hours, minutes, and seconds
    let min_hours = (min_time / 3600) % 24;
    let min_minutes = (min_time / 60) % 60;
    let min_seconds = min_time % 60;

    let max_hours = (max_time / 3600) % 24;
    let max_minutes = (max_time / 60) % 60;
    let max_seconds = max_time % 60;

    // Output the results in the specified format
    println!("{:02}:{:02}:{:02} {:02}:{:02}:{:02}", 
             min_hours, min_minutes, min_seconds, 
             max_hours, max_minutes, max_seconds);
}
