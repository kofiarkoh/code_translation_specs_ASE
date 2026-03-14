use std::io::{self, Write};

fn main() {
    loop {
        // Read input values for total seconds, hours, and seconds
        let mut input = String::new();
        print!("Enter total seconds, hours, and seconds (or -1 to exit): ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check for termination condition
        if values.len() == 3 && values[0] == -1 && values[1] == -1 && values[2] == -1 {
            break;
        }

        // Ensure we have exactly three inputs
        if values.len() != 3 {
            continue; // Skip invalid input
        }

        let total_seconds = values[0];
        let hours = values[1];
        let seconds = values[2];

        // Compute remaining time until a reference point
        let remaining_time = compute_remaining_time(total_seconds, hours, seconds);
        let one_third_time = remaining_time / 3;

        // Output formatted time strings
        let remaining_time_str = format_time(remaining_time);
        let one_third_time_str = format_time(one_third_time);

        println!("Remaining time: {}", remaining_time_str);
        println!("One third of remaining time: {}", one_third_time_str);
    }
}

// Function to compute remaining time based on input values
fn compute_remaining_time(total_seconds: i32, hours: i32, seconds: i32) -> i32 {
    // Here we assume a reference point of 0 for simplicity
    // The remaining time is calculated as total_seconds - (hours * 3600 + seconds)
    let input_time = hours * 3600 + seconds;
    total_seconds - input_time
}

// Function to format time into HH:MM:SS
fn format_time(total_seconds: i32) -> String {
    // Ensure non-negative time for formatting
    let total_seconds = total_seconds.max(0);
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
