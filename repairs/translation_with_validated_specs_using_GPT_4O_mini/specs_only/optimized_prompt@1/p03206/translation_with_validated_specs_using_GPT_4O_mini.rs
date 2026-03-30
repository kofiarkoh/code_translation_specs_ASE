use std::io::{self, BufRead};

fn holiday_message(day: i32) -> Option<&'static str> {
    // Match the input day to the corresponding holiday message
    match day {
        25 => Some("Christmas"),
        24 => Some("Christmas Eve"),
        23 => Some("Christmas Eve Eve"),
        22 => Some("Christmas Eve Eve Eve"),
        _ => None, // No output for any other integer values
    }
}

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Parse the input as an integer
    if let Ok(input_day) = input.trim().parse::<i32>() {
        if let Some(message) = holiday_message(input_day) {
            println!("{}", message); // Output the holiday message if it exists
        }
        // No output occurs for other values, as specified
    }
}
