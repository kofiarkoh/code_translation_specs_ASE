use std::io::{self, Write};

fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let (mut year, mut month) = if month == 1 || month == 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };
    
    (year + year / 4 - year / 100 + year / 400 + (13 * month + 8) / 5 + day) % 7
}

fn main() {
    let mut input = String::new();
    let days = [
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
    ];

    loop {
        input.clear();
        print!("Enter month and day (0 to exit): ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.split_whitespace();
        
        let m: i32 = match parts.next() {
            Some(num) => num.parse().unwrap_or(0),
            None => 0,
        };
        let d: i32 = match parts.next() {
            Some(num) => num.parse().unwrap_or(0),
            None => 0,
        };

        if m == 0 || d == 0 {
            break;
        }

        // Output the corresponding day of the week
        let day_index = day_of_week(2004, m, d);
        println!("{}", days[day_index as usize]);
    }
}
