use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok());

        let a = match nums.next() {
            Some(num) => num,
            None => continue, // Skip if no numbers are found
        };
        let b = match nums.next() {
            Some(num) => num,
            None => continue, // Skip if no numbers are found
        };

        // Termination condition
        if a == 0 && b == 0 {
            break;
        }

        let mut found_leap_year = false;

        for year in a..=b {
            if year % 400 == 0 {
                println!("{}", year);
                found_leap_year = true;
            } else if year % 100 == 0 {
                continue; // Not a leap year
            } else if year % 4 == 0 {
                println!("{}", year);
                found_leap_year = true;
            }
        }

        if !found_leap_year {
            println!("NA");
        }
        println!(); // Print a newline after each range
    }
}
