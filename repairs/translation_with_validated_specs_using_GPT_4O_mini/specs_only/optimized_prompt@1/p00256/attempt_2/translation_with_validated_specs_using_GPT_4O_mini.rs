use std::str::FromStr;

fn main() {
    // Example input for testing
    let input_string = "2023.10.05"; // Change this to test different inputs
    let result = convert_date(input_string);
    println!("{}", result);
}

fn convert_date(input: &str) -> String {
    // Check for the '#' character at the start of the input
    if input.starts_with('#') {
        return String::new(); // Terminate without processing
    }

    // Try to parse the input as a string date format
    if let Some((year, month, day)) = parse_string_date(input) {
        // Convert string date to numerical format
        return format_numerical_date(year, month, day);
    }

    // Try to parse the input as five integers
    if let Some((year, month, day)) = parse_numerical_date(input) {
        // Convert numerical date to string format
        return format_string_date(year, month, day);
    }

    // If input is not valid, return an empty string (as per preconditions)
    String::new()
}

fn parse_string_date(input: &str) -> Option<(i32, u32, u32)> {
    // Split the input string by '.' and parse the components
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() != 3 {
        return None; // Not a valid date format
    }

    let year = i32::from_str(parts[0]).ok()?;
    let month = u32::from_str(parts[1]).ok()?;
    let day = u32::from_str(parts[2]).ok()?;

    // Validate month and day ranges
    if month < 1 || month > 12 || day < 1 || day > days_in_month(year, month) {
        return None; // Invalid date
    }

    Some((year, month, day))
}

fn parse_numerical_date(input: &str) -> Option<(i32, u32, u32)> {
    // Split the input string by whitespace and parse the components
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 5 {
        return None; // Not a valid numerical format
    }

    let year = i32::from_str(parts[0]).ok()?;
    let month = u32::from_str(parts[1]).ok()?;
    let day = u32::from_str(parts[2]).ok()?;

    // Validate month and day ranges
    if month < 1 || month > 12 || day < 1 || day > days_in_month(year, month) {
        return None; // Invalid date
    }

    Some((year, month, day))
}

fn format_numerical_date(year: i32, month: u32, day: u32) -> String {
    // Calculate total days since January 1, 2001
    let total_days = days_since_reference(year, month, day);
    // Return the formatted string
    format!("{} {} {} {} {}", total_days, month, day, 0, 0) // Placeholder for additional values
}

fn format_string_date(year: i32, month: u32, day: u32) -> String {
    // Return the formatted string
    format!("{}.{}.{}", year, month, day)
}

fn days_since_reference(year: i32, month: u32, day: u32) -> i32 {
    // Calculate the total number of days since January 1, 2001
    let mut total_days = 0;

    // Add days for complete years
    for y in 2001..year {
        total_days += if is_leap_year(y) { 366 } else { 365 };
    }

    // Add days for complete months in the current year
    for m in 1..month {
        total_days += days_in_month(year, m) as i32; // Cast to i32 for addition
    }

    // Add days for the current month
    total_days += day as i32; // Cast to i32 for addition

    total_days
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0, // Invalid month
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
