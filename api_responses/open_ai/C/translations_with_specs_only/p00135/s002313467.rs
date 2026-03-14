fn main() {
    use std::io::{self, BufRead};

    // Read the number of time entries
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of time entries
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Expected a positive integer");

    // Process each time entry
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let time_parts: Vec<&str> = line.split_whitespace().collect();
            if time_parts.len() != 2 {
                // Undefined behavior for invalid time entries, just continue
                continue;
            }

            // Parse hours and minutes
            let hours: i32 = time_parts[0].parse().unwrap_or(-1);
            let minutes: i32 = time_parts[1].parse().unwrap_or(-1);

            // Check for valid hour and minute values
            if hours < 0 || hours > 12 || minutes < 0 || minutes >= 60 {
                // Undefined behavior for invalid time entries, just continue
                continue;
            }

            // Calculate the angle between the hour and minute hands
            let hour_angle = (hours % 12) * 30 + (minutes / 2);
            let minute_angle = minutes * 6;
            let angle = (hour_angle - minute_angle).abs();
            let angle = if angle > 180 { 360 - angle } else { angle };

            // Determine the alert based on the angle
            let alert = if angle < 30 {
                "alert"
            } else if angle < 150 {
                "warning"
            } else {
                "safe"
            };

            // Output the alert
            println!("{}", alert);
        }
    }
}
