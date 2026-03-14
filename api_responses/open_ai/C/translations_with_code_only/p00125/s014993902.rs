use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;
use chrono::{Datelike, Timelike, TimeZone, NaiveDate};

fn main() {
    let mut input = String::new();

    loop {
        input.clear();
        // Read input from stdin
        if io::stdin().read_line(&mut input).is_err() {
            break; // Exit on read error
        }

        let mut parts = input.split_whitespace();
        let year1: i32 = match parts.next() {
            Some(part) => part.parse().unwrap_or(-1),
            None => -1,
        };
        let month1: i32 = match parts.next() {
            Some(part) => part.parse().unwrap_or(-1),
            None => -1,
        };
        let day1: i32 = match parts.next() {
            Some(part) => part.parse().unwrap_or(-1),
            None => -1,
        };
        let year2: i32 = match parts.next() {
            Some(part) => part.parse().unwrap_or(-1),
            None => -1,
        };
        let month2: i32 = match parts.next() {
            Some(part) => part.parse().unwrap_or(-1),
            None => -1,
        };
        let day2: i32 = match parts.next() {
            Some(part) => part.parse().unwrap_or(-1),
            None => -1,
        };

        // Check for termination condition
        if month1 == -1 || year1 == -1 || day1 == -1 || month2 == -1 || year2 == -1 || day2 == -1 {
            break;
        }

        // Create NaiveDate for the first date
        let date1 = NaiveDate::from_ymd(year1, month1 as u32, day1 as u32);
        // Create NaiveDate for the second date
        let date2 = NaiveDate::from_ymd(year2, month2 as u32, day2 as u32);

        // Calculate the difference in days
        let diff = (date2 - date1).num_days();

        // Print the result, rounding as in the original C code
        println!("{}", diff);
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The Rust code reads input line by line and splits it into parts, similar to how `scanf` works in C. It uses `unwrap_or(-1)` to handle parsing errors, mimicking the behavior of the original C code.
  
2. **Date Handling**: The `chrono` crate is used to handle dates. This is a safe and idiomatic way to manage date and time in Rust, avoiding the need for unsafe code. The `NaiveDate` type is used to represent dates without timezone information, similar to how `struct tm` is used in C.

3. **Loop and Termination**: The loop continues until a termination condition is met, which is directly translated from the C code.

4. **Difference Calculation**: The difference in days is calculated using the `num_days()` method, which directly corresponds to the `difftime` and subsequent division in the C code.

5. **Output**: The output is formatted using `println!`, preserving the newline behavior of the original C code. 

This translation maintains the semantics of the original C code while adhering to Rust's safety and idiomatic practices.
