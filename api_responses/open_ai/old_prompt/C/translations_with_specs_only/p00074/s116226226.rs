use std::io::{self, Write};

fn main() {
    loop {
        // Prompt the user for input
        print!("Enter time (HH MM SS) or -1 -1 -1 to exit: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input into hours, minutes, and seconds
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            continue; // Ignore invalid input formats
        }

        // Attempt to parse the input as integers
        let h: i32 = match parts[0].parse() {
            Ok(val) if val >= 0 && val <= 23 => val,
            _ => continue, // Ignore invalid hours
        };
        let m: i32 = match parts[1].parse() {
            Ok(val) if val >= 0 && val <= 59 => val,
            _ => continue, // Ignore invalid minutes
        };
        let s: i32 = match parts[2].parse() {
            Ok(val) if val >= 0 && val <= 59 => val,
            _ => continue, // Ignore invalid seconds
        };

        // Check for termination condition
        if h == -1 && m == -1 && s == -1 {
            break; // Exit the loop
        }

        // Calculate total seconds until two hours
        let total_seconds = (h * 3600 + m * 60 + s) % 7200; // Wrap around at 7200 seconds (2 hours)
        let remaining_seconds = 7200 - total_seconds; // Remaining seconds until two hours

        // Calculate hours, minutes, and seconds for output
        let remaining_h = remaining_seconds / 3600;
        let remaining_m = (remaining_seconds % 3600) / 60;
        let remaining_s = remaining_seconds % 60;

        // Format and print the remaining time
        println!(
            "{:02}:{:02}:{:02}",
            remaining_h, remaining_m, remaining_s
        );

        // Print the remaining time multiplied by three
        let multiplied_remaining_seconds = remaining_seconds * 3;
        let multiplied_h = multiplied_remaining_seconds / 3600;
        let multiplied_m = (multiplied_remaining_seconds % 3600) / 60;
        let multiplied_s = multiplied_remaining_seconds % 60;

        // Format and print the multiplied remaining time
        println!(
            "{:02}:{:02}:{:02}",
            multiplied_h, multiplied_m, multiplied_s
        );
    }
}
