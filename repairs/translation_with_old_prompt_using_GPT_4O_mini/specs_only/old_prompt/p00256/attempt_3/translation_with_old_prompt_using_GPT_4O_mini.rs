use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any surrounding whitespace
    let input = input.trim();

    // Check for the invalid input case where the input starts with '#'
    if input.starts_with('#') {
        return; // Do nothing and exit
    }

    // Check if the input is in the "YYYY.MM.DD" format
    if let Some((year, month, day)) = parse_date_string(input) {
        // Output in the format "M.D.D.D"
        println!("{}.{}.{}.{}", month, day, day, day); // Corrected to match the specification
    } else {
        // Attempt to parse as a series of integers
        let parts: Vec<&str> = input.split('.').collect();
        if parts.len() > 0 {
            // Handle the case where input is a series of integers
            let year = parts[0].parse::<i32>().unwrap_or(0);
            let month = if parts.len() > 1 {
                parts[1].parse::<i32>().unwrap_or(0)
            } else {
                0
            };
            let day = if parts.len() > 2 {
                parts[2].parse::<i32>().unwrap_or(0)
            } else {
                0
            };
            // Output in the format "YYYY.M.D"
            println!("{}{}.{}", year, month, day);
        }
    }
}

// Function to parse the date from a string in the format "YYYY.MM.DD"
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
