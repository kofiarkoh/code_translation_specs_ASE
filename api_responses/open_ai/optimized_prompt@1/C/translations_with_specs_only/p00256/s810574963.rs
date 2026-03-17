use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {
    // Validates the date according to Gregorian calendar rules
    fn is_valid(&self) -> bool {
        if self.year < 1 || self.month < 1 || self.month > 12 || self.day < 1 {
            return false;
        }
        let days_in_month = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0) {
                    29 // Leap year
                } else {
                    28
                }
            }
            _ => return false,
        };
        self.day <= days_in_month
    }

    // Converts a date from "YYYY.MM.DD" format to a Date struct
    fn from_standard_format(date_str: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = date_str.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid date format");
        }
        let year = i32::from_str(parts[0]).map_err(|_| "Invalid year")?;
        let month = u32::from_str(parts[1]).map_err(|_| "Invalid month")?;
        let day = u32::from_str(parts[2]).map_err(|_| "Invalid day")?;
        let date = Date { year, month, day };
        if date.is_valid() {
            Ok(date)
        } else {
            Err("Invalid date")
        }
    }

    // Converts a date from custom encoded format to a Date struct
    fn from_custom_format(encoded_str: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = encoded_str.split('.').collect();
        if parts.len() != 5 {
            return Err("Invalid encoded format");
        }
        let year = i32::from_str(parts[0]).map_err(|_| "Invalid year")?;
        let month = u32::from_str(parts[1]).map_err(|_| "Invalid month")?;
        let day = u32::from_str(parts[2]).map_err(|_| "Invalid day")?;
        // The last two parts are ignored as per the specification
        let date = Date { year, month, day };
        if date.is_valid() {
            Ok(date)
        } else {
            Err("Invalid date")
        }
    }

    // Outputs the date in "YYYY.MM.DD" format
    fn to_standard_format(&self) -> String {
        format!("{:04}.{:02}.{:02}", self.year, self.month, self.day)
    }

    // Outputs the date in "YYYY.MM.DD.DD.DD" format
    fn to_custom_format(&self) -> String {
        format!("{:04}.{:02}.{:02}.00.00", self.year, self.month, self.day) // Last two parts are fixed as "00"
    }
}

fn main() {
    let input = "2023.10.05"; // Example input, can be changed to test different cases

    if input.is_empty() || input.starts_with('#') {
        return; // Invalid input based on preconditions
    }

    let output = if input.matches('.').count() == 2 {
        // Standard format
        match Date::from_standard_format(input) {
            Ok(date) => date.to_standard_format(),
            Err(_) => return, // Handle invalid date
        }
    } else if input.matches('.').count() == 4 {
        // Custom encoded format
        match Date::from_custom_format(input) {
            Ok(date) => date.to_custom_format(),
            Err(_) => return, // Handle invalid date
        }
    } else {
        return; // Invalid format
    };

    println!("{}", output); // Output the result
}
