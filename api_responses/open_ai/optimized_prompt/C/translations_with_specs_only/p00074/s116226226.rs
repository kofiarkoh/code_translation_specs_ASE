use std::io::{self, Write};

fn main() {
    loop {
        // Read input from the user
        let mut input = String::new();
        print!("Enter hours, minutes, and seconds (or -1 to exit): ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input into three integers
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            continue; // Ignore invalid input length
        }

        let hours: i32 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };
        let minutes: i32 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };
        let seconds: i32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };

        // Check for termination signal
        if hours == -1 || minutes == -1 || seconds == -1 {
            break; // Exit the loop on termination signal
        }

        // Validate time components
        if hours < 0 || minutes < 0 || seconds < 0 || minutes >= 60 || seconds >= 60 {
            continue; // Ignore invalid time components
        }

        // Calculate total seconds from input time
        let total_seconds = hours * 3600 + minutes * 60 + seconds;

        // Calculate the next two-hour mark in seconds
        let next_two_hour_mark = ((total_seconds + 7200) / 7200) * 7200;

        // Calculate remaining time until the next two-hour mark
        let remaining_time = next_two_hour_mark - total_seconds;

        // Calculate three times the remaining time
        let three_times_remaining = remaining_time * 3;

        // Convert remaining time to hours, minutes, and seconds
        let (remaining_hours, remaining_minutes, remaining_seconds) = convert_seconds_to_hms(remaining_time);
        let (three_hours, three_minutes, three_seconds) = convert_seconds_to_hms(three_times_remaining);

        // Output the results
        println!("{:02}:{:02}:{:02}", remaining_hours, remaining_minutes, remaining_seconds);
        println!("{:02}:{:02}:{:02}", three_hours, three_minutes, three_seconds);
    }
}

// Helper function to convert seconds to hours, minutes, and seconds
fn convert_seconds_to_hms(total_seconds: i32) -> (i32, i32, i32) {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    (hours, minutes, seconds)
}
