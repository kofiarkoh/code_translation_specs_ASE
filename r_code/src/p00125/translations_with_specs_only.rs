use std::io::{self, BufRead};
use chrono::{Datelike, NaiveDate};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<i32> = line.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        // Check for termination condition
        if values.iter().any(|&x| x == -1) {
            break;
        }

        // Unpack the values into respective date components
        let (year1, month1, day1, year2, month2, day2) = (
            values[0], values[1], values[2],
            values[3], values[4], values[5],
        );

        // Create NaiveDate instances for both dates
        let date1 = NaiveDate::from_ymd(year1, month1 as u32, day1 as u32);
        let date2 = NaiveDate::from_ymd(year2, month2 as u32, day2 as u32);

        // Calculate the number of days between the two dates
        let duration = date2.signed_duration_since(date1).num_days();

        // Output the result
        println!("{}", duration);
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input and expects a specific format. It splits the input line by commas and parses the values into integers.
2. **Termination Condition**: The program checks for `-1` in the input values to terminate processing, as specified.
3. **Date Handling**: The `chrono` crate is used for date manipulation, which provides safe and reliable date handling. The `NaiveDate::from_ymd` method is used to create date instances.
4. **Duration Calculation**: The `signed_duration_since` method calculates the difference in days between two dates, which aligns with the requirement to output the number of days between the two dates.
5. **Output**: Each result is printed immediately after processing the input pair, adhering to the specification's output requirements.
