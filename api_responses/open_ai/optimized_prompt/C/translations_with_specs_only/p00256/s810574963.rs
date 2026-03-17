use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim whitespace and check for invalid starting character
    let input = input.trim();
    if input.is_empty() || input.starts_with('#') {
        return; // No output for invalid input
    }

    // Attempt to parse the input as a standard date format
    if let Ok(date) = parse_standard_date(input) {
        // If parsing is successful, print in standard format
        println!("{}", date);
    } else if let Ok(encoded_date) = parse_encoded_date(input) {
        // If parsing as encoded format is successful, print in encoded format
        println!("{}", encoded_date);
    }
    // If neither parsing succeeds, the behavior is undefined as per specification
}

// Parses a date in "YYYY.MM.DD" format
fn parse_standard_date(input: &str) -> Result<String, ()> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() != 3 {
        return Err(()); // Invalid format
    }

    let year: i32 = parts[0].parse().map_err(|_| ())?;
    let month: u32 = parts[1].parse().map_err(|_| ())?;
    let day: u32 = parts[2].parse().map_err(|_| ())?;

    // Validate the date according to Gregorian calendar rules
    if is_valid_date(year, month, day) {
        Ok(format!("{}.{}.{}", year, month, day))
    } else {
        Err(()) // Invalid date
    }
}

// Parses a custom encoded date format consisting of five integers
fn parse_encoded_date(input: &str) -> Result<String, ()> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() != 5 {
        return Err(()); // Invalid format
    }

    let year: i32 = parts[0].parse().map_err(|_| ())?;
    let month: u32 = parts[1].parse().map_err(|_| ())?;
    let day: u32 = parts[2].parse().map_err(|_| ())?;
    let extra1: u32 = parts[3].parse().map_err(|_| ())?;
    let extra2: u32 = parts[4].parse().map_err(|_| ())?;

    // Validate the date according to Gregorian calendar rules
    if is_valid_date(year, month, day) {
        Ok(format!("{}.{}.{}.{}.{}", year, month, day, extra1, extra2))
    } else {
        Err(()) // Invalid date
    }
}

// Validates a date according to the Gregorian calendar
fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
    if month < 1 || month > 12 || day < 1 {
        return false; // Month or day out of range
    }

    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => return false, // Invalid month
    };

    day <= days_in_month // Check if day is valid for the month
}

// Determines if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
