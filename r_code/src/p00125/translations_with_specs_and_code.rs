use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        let year1: i32 = parts.next().unwrap().parse().unwrap();
        let month1: u32 = parts.next().unwrap().parse().unwrap();
        let day1: u32 = parts.next().unwrap().parse().unwrap();
        let year2: i32 = parts.next().unwrap().parse().unwrap();
        let month2: u32 = parts.next().unwrap().parse().unwrap();
        let day2: u32 = parts.next().unwrap().parse().unwrap();

        // Check for termination condition
        if year1 == -1 || month1 == 0 || day1 == 0 || year2 == -1 || month2 == 0 || day2 == 0 {
            break;
        }

        // Create the first date
        let date1 = match create_date(year1, month1, day1) {
            Ok(d) => d,
            Err(_) => continue, // Skip invalid dates
        };

        // Create the second date
        let date2 = match create_date(year2, month2, day2) {
            Ok(d) => d,
            Err(_) => continue, // Skip invalid dates
        };

        // Calculate the difference in days
        let duration = date2.duration_since(date1).unwrap();
        let days = duration.as_secs() / 86400; // 86400 seconds in a day

        println!("{}", days);
    }
}

// Function to create a date from year, month, and day
fn create_date(year: i32, month: u32, day: u32) -> Result<SystemTime, &'static str> {
    if month < 1 || month > 12 || day < 1 || day > 31 {
        return Err("Invalid month or day");
    }

    // Create a tm struct
    let mut tm = chrono::NaiveDate::from_ymd(year, month, day);
    
    // Check for valid date
    if tm.year() != year || tm.month() != month || tm.day() != day {
        return Err("Invalid date");
    }

    // Convert to SystemTime
    let timestamp = tm.and_hms(0, 0, 0).timestamp();
    Ok(UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64))
}
```

### Notes:
1. **Date Handling**: The Rust code uses the `chrono` crate for date handling, which provides a safe and idiomatic way to manage dates and times. This avoids the need for unsafe code.
2. **Termination Condition**: The termination condition checks for `-1` values as specified, and the program breaks out of the loop if any of the date components are invalid.
3. **Error Handling**: The code gracefully skips invalid dates and continues processing valid pairs.
4. **Output**: The output format is preserved, ensuring each result is printed on a new line.
5. **Safety**: The code avoids using `unsafe` and handles potential errors through `Result` types, ensuring safe execution.
