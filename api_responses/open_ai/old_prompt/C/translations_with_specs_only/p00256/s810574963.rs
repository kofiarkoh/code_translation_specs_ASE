use std::io::{self, Write};

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any surrounding whitespace
    let input = input.trim();

    // Check for the invalid starting character
    if input.starts_with('#') {
        return; // Do nothing if the input starts with '#'
    }

    // Check if the input is in the "YYYY.MM.DD" format
    if let Some((year, month, day)) = parse_date_string(input) {
        // Output in the format "M.D.D.D.D"
        println!("{}.{}.{}.{}", month, day, day, day);
    } else if let Some((year, month, day)) = parse_date_integers(input) {
        // Output in the format "YYYY.M.D"
        println!("{}{}.{}", year, month, day);
    } else {
        // If the input format is invalid, the behavior is undefined as per the specification
        // The program may enter an infinite loop or produce incorrect output
        // No explicit error handling is provided
    }
}

// Function to parse date from string format "YYYY.MM.DD"
fn parse_date_string(s: &str) -> Option<(i32, i32, i32)> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() == 3 {
        if let (Ok(year), Ok(month), Ok(day)) = (
            parts[0].parse::<i32>(),
            parts[1].parse::<i32>(),
            parts[2].parse::<i32>(),
        ) {
            return Some((year, month, day));
        }
    }
    None
}

// Function to parse date from integer format
fn parse_date_integers(s: &str) -> Option<(i32, i32, i32)> {
    // This function assumes the input is a space-separated string of integers
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() == 3 {
        if let (Ok(year), Ok(month), Ok(day)) = (
            parts[0].parse::<i32>(),
            parts[1].parse::<i32>(),
            parts[2].parse::<i32>(),
        ) {
            return Some((year, month, day));
        }
    }
    None
}
