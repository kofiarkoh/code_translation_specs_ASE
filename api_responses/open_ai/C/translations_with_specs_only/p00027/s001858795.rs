use chrono::{Datelike, NaiveDate, Weekday};

fn main() {
    let year = 2004; // Fixed year as per specification

    loop {
        let mut month_input = String::new();
        let mut day_input = String::new();

        // Read month input
        println!("Enter month (1-12, 0 to exit):");
        std::io::stdin().read_line(&mut month_input).unwrap();
        let month: u32 = month_input.trim().parse().unwrap_or(0);

        // Check for termination condition
        if month == 0 {
            break;
        }

        // Read day input
        println!("Enter day (valid day for the month, 0 to exit):");
        std::io::stdin().read_line(&mut day_input).unwrap();
        let day: u32 = day_input.trim().parse().unwrap_or(0);

        // Check for termination condition
        if day == 0 {
            break;
        }

        // Create date and determine the day of the week
        if let Ok(date) = NaiveDate::from_ymd_opt(year, month, day) {
            let weekday = date.weekday();
            println!("The day of the week is: {:?}", weekday);
        } else {
            // If the date is invalid, we assume valid input as per specification
            // No explicit error handling is required
            println!("Invalid date input, but we assume valid inputs as per spec.");
        }
    }
}
```

### Explanation of Choices:
- **Fixed Year**: The year is hardcoded to 2004 as specified.
- **Input Handling**: The program reads inputs in a loop until a zero is entered for either month or day, which aligns with the termination condition.
- **Date Validation**: The `NaiveDate::from_ymd_opt` method is used to create a date safely. If the date is invalid, it simply prints a message, but the specification assumes valid inputs.
- **Day of the Week Calculation**: The `weekday()` method from the `chrono` crate is used to get the day of the week, which is consistent with the Gregorian calendar.
- **No Unsafe Code**: The implementation is entirely safe and idiomatic Rust, adhering to the requirement to avoid `unsafe` unless necessary.
