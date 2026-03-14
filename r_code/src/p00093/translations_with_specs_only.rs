use std::io::{self, BufRead};

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 2 {
            continue; // Assumes valid input as per specification
        }

        let (start, end) = (nums[0], nums[1]);

        // Termination condition
        if start == 0 && end == 0 {
            break;
        }

        // Ensure valid range
        if start > end {
            continue; // Assumes valid input as per specification
        }

        let mut found_leap_year = false;

        for year in start..=end {
            if is_leap_year(year) {
                println!("{}", year);
                found_leap_year = true;
            }
        }

        if !found_leap_year {
            println!("NA");
        }

        println!(); // Output a newline after each range
    }
}
