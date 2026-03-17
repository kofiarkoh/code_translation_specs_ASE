use std::io::{self, Write};

fn main() {
    loop {
        // Read input from the user
        let mut input = String::new();
        print!("Enter time (HH MM SS) or -1 -1 -1 to exit: ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            continue; // Ignore invalid input
        }

        let h: i32 = match parts[0].parse() {
            Ok(num) if num >= 0 && num <= 23 => num,
            _ => continue, // Ignore invalid hour
        };

        let m: i32 = match parts[1].parse() {
            Ok(num) if num >= 0 && num <= 59 => num,
            _ => continue, // Ignore invalid minute
        };

        let s: i32 = match parts[2].parse() {
            Ok(num) if num >= 0 && num <= 59 => num,
            _ => continue, // Ignore invalid second
        };

        // Check for termination condition
        if h == -1 && m == -1 && s == -1 {
            break; // Exit the loop
        }

        // Calculate current seconds and remaining time
        let cur_sec = h * 3600 + m * 60 + s;
        let remaining = 2 * 3600 - cur_sec;

        // Output remaining time in HH:MM:SS format
        print_time(remaining);

        // Calculate remaining time multiplied by three and output
        let remaining_tripled = remaining * 3;
        print_time(remaining_tripled);
    }
}

// Function to print time in HH:MM:SS format
fn print_time(total_seconds: i32) {
    let hours = (total_seconds / 3600).rem_euclid(24); // Use rem_euclid to handle negative values
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}
