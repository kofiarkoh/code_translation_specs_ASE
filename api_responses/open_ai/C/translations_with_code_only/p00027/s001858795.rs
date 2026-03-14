use std::io::{self, Write};

fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let mut year = year;
    let mut month = month;

    // Adjust year and month for January and February
    if month == 1 || month == 2 {
        year -= 1;
        month += 12;
    }

    // Calculate the day of the week using the formula
    (year + year / 4 - year / 100 + year / 400 + (13 * month + 8) / 5 + day) % 7
}

fn main() {
    let days = [
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
    ];

    loop {
        let mut input = String::new();
        // Read input from stdin
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        
        // Parse month and day
        let m: i32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(value) => value,
            None => continue,
        };
        let d: i32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(value) => value,
            None => continue,
        };

        // Break if either month or day is 0
        if m == 0 || d == 0 {
            break;
        }

        // Output the corresponding day of the week
        let day_index = day_of_week(2004, m, d);
        println!("{}", days[day_index as usize]);
    }
}
