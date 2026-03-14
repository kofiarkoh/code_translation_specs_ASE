use std::io::{self, BufRead};

fn main() {
    let mut card = [0; 11]; // Create an array of 11 integers
    for i in 1..11 {
        card[i] = 1; // Initialize card array elements to 1
    }

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the result of reading a line
        let mut numbers = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok());

        // Read three integers from the input
        let c1 = match numbers.next() {
            Some(num) => num,
            None => break, // Exit if not enough numbers are provided
        };
        let c2 = match numbers.next() {
            Some(num) => num,
            None => break,
        };
        let c3 = match numbers.next() {
            Some(num) => num,
            None => break,
        };

        let mut cnt = 0;
        for i in 1..11 {
            if c1 != i && c2 != i && c3 != i {
                if c1 + c2 + i <= 20 {
                    cnt += 1; // Increment count if conditions are met
                }
            }
        }

        if cnt >= 4 {
            println!("YES"); // Output YES if count is 4 or more
        } else {
            println!("NO"); // Output NO otherwise
        }
    }
}
