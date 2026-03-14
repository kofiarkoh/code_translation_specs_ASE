use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut nums = input.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());

        let a = match nums.next() {
            Some(num) => num,
            None => continue,
        };
        let b = match nums.next() {
            Some(num) => num,
            None => continue,
        };

        if a == 0 && b == 0 {
            break;
        }

        let mut judge = 0;
        let mut found = false;

        for year in a..=b {
            if year % 400 == 0 {
                println!("{}", year);
                judge = 1;
                found = true;
            } else if year % 100 == 0 {
                continue;
            } else if year % 4 == 0 {
                println!("{}", year);
                judge = 1;
                found = true;
            }
        }

        if judge == 0 {
            println!("NA");
        }
        println!(); // Print a newline after each case
    }
}
