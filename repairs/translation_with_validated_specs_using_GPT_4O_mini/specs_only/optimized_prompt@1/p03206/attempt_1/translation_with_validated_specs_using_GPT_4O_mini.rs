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
    // Example usage: replace with actual input handling as needed
    let input_day = 24; // This would be replaced with actual input logic
    if let Some(message) = holiday_message(input_day) {
        println!("{}", message); // Output the holiday message if it exists
    }
    // No output occurs for other values, as specified
}
